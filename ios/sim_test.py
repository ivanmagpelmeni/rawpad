#!/usr/bin/env python3
"""Drive RawPad in the booted iPad simulator over its control channel and take real screenshots.

Usage: sim_test.py <tour.jsonl> <out_dir> [port]
Exits non-zero when the app can't be reached, a request with "expectPhotosAtLeast" fails, or the app is gone
at the end. Every reply is written to <out_dir>/replies.jsonl.
"""
import json
import os
import socket
import subprocess
import sys
import time

BUNDLE = "io.github.ivanmagpelmeni.rawpad"


def connect(port, timeout=120):
    deadline = time.time() + timeout
    last = None
    while time.time() < deadline:
        try:
            s = socket.create_connection(("127.0.0.1", port), timeout=5)
            s.settimeout(90)
            return s
        except OSError as e:
            last = e
            time.sleep(2)
    raise SystemExit(f"control channel never came up on port {port}: {last}")


def request(sock, reader, method, params):
    sock.sendall((json.dumps({"method": method, "params": params}) + "\n").encode())
    line = reader.readline()
    if not line:
        raise SystemExit(f"connection closed during {method} (did the app crash?)")
    return json.loads(line)


def app_running():
    out = subprocess.run(["xcrun", "simctl", "spawn", "booted", "launchctl", "list"], capture_output=True, text=True).stdout
    return BUNDLE in out


def log_size(out_dir):
    try:
        return os.path.getsize(os.path.join(out_dir, "app-stderr.log"))
    except OSError:
        return 0


def log_text(out_dir, since):
    try:
        with open(os.path.join(out_dir, "app-stderr.log"), "rb") as f:
            f.seek(since)
            return f.read().decode("utf-8", "replace")
    except OSError:
        return ""


def app_data():
    return subprocess.run(["xcrun", "simctl", "get_app_container", "booted", BUNDLE, "data"], capture_output=True, text=True).stdout.strip()


def app_rss_mb():
    """Resident memory of the app process in MB (simulator apps are ordinary Mac processes), or None."""
    try:
        pids = subprocess.run(["pgrep", "-f", "RawPad.app/rawpad"], capture_output=True, text=True).stdout.split()
        if not pids:
            return None
        rss = subprocess.run(["ps", "-o", "rss=", "-p", pids[0]], capture_output=True, text=True).stdout.strip()
        return round(int(rss) / 1024, 1)
    except (OSError, ValueError):
        return None


def substitute(value, names):
    """Replace "$name" strings (whole string, or a "$APPDATA/..." prefix) in request params."""
    if isinstance(value, dict):
        return {k: substitute(v, names) for k, v in value.items()}
    if isinstance(value, list):
        return [substitute(v, names) for v in value]
    if isinstance(value, str) and value.startswith("$"):
        head, _, rest = value[1:].partition("/")
        if head in names:
            got = names[head]
            return f"{got}/{rest}" if rest else got
    return value


def dig(value, path):
    for key in path.split("."):
        if isinstance(value, list):
            try:
                value = value[int(key)] if key.isdigit() else (value[0] if value else None)
            except IndexError:
                value = None
        else:
            value = value.get(key) if isinstance(value, dict) else None
    return value


def main():
    tour, out_dir = sys.argv[1], sys.argv[2]
    port = int(sys.argv[3]) if len(sys.argv) > 3 else 7980
    sock = connect(port)
    reader = sock.makefile("r", encoding="utf-8")
    failures = []
    warnings = []
    marks = {}
    names = {"APPDATA": app_data()}
    with open(f"{out_dir}/replies.jsonl", "w") as log:
        for raw in open(tour):
            raw = raw.strip()
            if not raw or raw.startswith("#"):
                continue
            step = json.loads(raw)
            if "ifFile" in step and not os.path.isfile(os.path.join(names["APPDATA"], step["ifFile"])):
                # {"ifFile": "tmp/x.arw", ...}: only when CI could put that file in the app's
                # container (a download that may fail); otherwise skipped with a warning
                print(f"WARN skipped (no {step['ifFile']}): {raw[:120]}")
                warnings.append(f"skipped, {step['ifFile']} missing: {raw[:120]}")
                continue
            if "params" in step:
                step["params"] = substitute(step["params"], names)
            if "memory" in step:
                # {"memory": "label", "maxRssMB": 2500}: record the app's memory (ui.inspect ›
                # memory, plus the process's resident size) in memory.jsonl
                reply = request(sock, reader, "ui.inspect", {})
                mem = (reply.get("result") or {}).get("memory")
                rss = app_rss_mb()
                with open(f"{out_dir}/memory.jsonl", "a") as m:
                    m.write(json.dumps({"label": step["memory"], "rssMB": rss, "memory": mem}) + "\n")
                engine = (mem or {}).get("engineBytes")
                limit = step.get("maxRssMB")
                good = rss is not None and (limit is None or rss <= limit)
                engine_mb = round(engine / 1048576, 1) if isinstance(engine, (int, float)) else None
                print(f"{'ok ' if good else ('WARN' if step.get('soft') else 'ERR')} memory {step['memory']}: rss {rss} MB (max {limit}), engine {engine_mb} MB")
                if rss is None:
                    # pgrep/ps found no process: a crash shows up in the tour's other checks
                    warnings.append(f"memory {step['memory']}: the app's resident size was unreadable")
                elif not good:
                    (warnings if step.get("soft") else failures).append(f"memory {step['memory']}: rss {rss} MB over {limit} MB")
                continue
            if "sleep" in step:
                time.sleep(step["sleep"])
                continue
            if "waitUntil" in step:
                # repeat a request until a dotted path in its result is truthy, e.g.
                # {"waitUntil": "export.last", "method": "ui.inspect", "timeout": 60}
                deadline = time.time() + step.get("timeout", 60)
                while True:
                    reply = request(sock, reader, step["method"], step.get("params", {}))
                    value = reply.get("result")
                    for key in step["waitUntil"].split("."):
                        if isinstance(value, list):
                            value = value[0] if value else None
                        value = value.get(key) if isinstance(value, dict) else None
                    if "equals" in step:
                        # {"waitUntil": "rating", "equals": 5}: until the value is exactly that
                        value = value == step["equals"]
                    elif "differs" in step:
                        # {"waitUntil": "ui.pan", "differs": [0.5, 0.5]}: until the value changes
                        value = value is not None and value != step["differs"]
                    elif "wider" in step:
                        # {"waitUntil": "window", "wider": true}: a [width, height] in landscape
                        # (false: in portrait), e.g. after a hostMenu rotation
                        wide = isinstance(value, list) and len(value) == 2 and value[0] > value[1]
                        value = wide == bool(step["wider"])
                    if value or time.time() > deadline:
                        break
                    time.sleep(2)
                log.write(json.dumps({"request": step, "reply": reply}) + "\n")
                print(f"{'ok ' if value else ('WARN' if step.get('soft') else 'ERR')} waitUntil {step['waitUntil']} = {json.dumps(value)[:200]}")
                if not value:
                    msg = f"{step['method']}: {step['waitUntil']} never became true: {json.dumps(reply)[:300]}"
                    # "soft": true only warns (simulator-only flakiness, e.g. photolibraryd; DECISIONS.md 38)
                    (warnings if step.get("soft") else failures).append(msg)
                continue
            if "hostKeys" in step:
                # type on the CI Mac into the Simulator window (a hardware-keyboard probe; never
                # used on a person's computer). Never fails the tour: the runner may lack
                # accessibility permission for System Events.
                using = " using command down" if step.get("cmd") else ""
                keys = step["hostKeys"]
                script = f'tell application "Simulator" to activate\ndelay 1\ntell application "System Events" to keystroke "{keys}"{using}'
                cmd = bool(step.get("cmd"))
                try:
                    r = subprocess.run(["osascript", "-e", script], capture_output=True, text=True, timeout=30)
                    status, err = ("ok " if r.returncode == 0 else "WARN"), r.stderr.strip()[:200]
                except (OSError, subprocess.TimeoutExpired) as e:
                    status, err = "WARN", str(e)[:200]
                print(f"{status} hostKeys {keys!r} cmd={cmd} {err}")
                continue
            if "hostMenu" in step:
                # {"hostMenu": ["Device", "Rotate Left"]}: pick a menu item of the Simulator app on
                # the CI Mac (rotation; simctl has no command for it). Same System Events access as
                # hostKeys; never fails the tour itself: the waitUntil after it checks the effect
                menu, item = step["hostMenu"]
                script = (
                    'tell application "Simulator" to activate\ndelay 1\n'
                    f'tell application "System Events" to tell process "Simulator" to click menu item "{item}" of menu "{menu}" of menu bar 1'
                )
                try:
                    r = subprocess.run(["osascript", "-e", script], capture_output=True, text=True, timeout=30)
                    status, err = ("ok " if r.returncode == 0 else "WARN"), r.stderr.strip()[:200]
                except (OSError, subprocess.TimeoutExpired) as e:
                    status, err = "WARN", str(e)[:200]
                print(f"{status} hostMenu {menu} > {item} {err}")
                continue
            if "logMark" in step:
                # remember how far the app's log (app-stderr.log, written by simctl launch) is
                marks[step["logMark"]] = log_size(out_dir)
                print(f"log mark {step['logMark']} at {marks[step['logMark']]}")
                continue
            if "checkLog" in step:
                # {"checkLog": "text", "since": "mark", "absent": true}: the app logged (or, with
                # absent, never logged) that text since the mark (e.g. the on-screen keyboard)
                # a present check polls for up to `wait` seconds: the app may not have flushed the
                # line yet (run 60's first attempt saw an empty log right after the tap)
                deadline = time.time() + step.get("wait", 5)
                while True:
                    time.sleep(1)
                    text = log_text(out_dir, marks.get(step.get("since"), 0))
                    found = step["checkLog"] in text
                    if found or step.get("absent") or time.time() >= deadline:
                        break
                good = (not found) if step.get("absent") else found
                what = f"checkLog {step['checkLog']!r} {'absent' if step.get('absent') else 'present'} since {step.get('since')}"
                print(f"{'ok ' if good else ('WARN' if step.get('soft') else 'ERR')} {what}")
                if not good:
                    (warnings if step.get("soft") else failures).append(f"{what}: log since the mark: {text[-600:]!r}")
                continue
            if "checkFiles" in step:
                # files the app wrote inside its container, e.g. Documents/Exports
                data = subprocess.run(["xcrun", "simctl", "get_app_container", "booted", BUNDLE, "data"], capture_output=True, text=True).stdout.strip()
                folder = os.path.join(data, step["checkFiles"])
                found = sorted(os.listdir(folder)) if os.path.isdir(folder) else []
                print(f"files in {step['checkFiles']}: {found}")
                if len(found) < step.get("min", 1):
                    failures.append(f"expected at least {step.get('min', 1)} file(s) in {step['checkFiles']}, found {found}")
                continue
            if "clickVisible" in step:
                # {"clickVisible": "thumb:", "count": 2, "exclude": ["thumb:1"]}: tap the first `count`
                # widgets whose id contains the text and whose rect lies fully on screen (top to bottom),
                # because which photo ids a grid shows depends on its sort and the screen size.
                # "skip": n passes over the first n, "button": "right" long-presses (the context menu),
                # "soft": true only warns when too few are on screen
                reply = request(sock, reader, "ui.widgets", {"filter": step["clickVisible"]})
                top, bottom = step.get("top", 50), step.get("bottom", 1130)
                seen = [
                    w for w in (reply.get("result") or [])
                    if w["id"] not in step.get("exclude", []) and w["rect"][1] >= top and w["rect"][1] + w["rect"][3] <= bottom
                ]
                seen.sort(key=lambda w: (w["rect"][1], w["rect"][0]))
                skip = step.get("skip", 0)
                picked = [w["id"] for w in seen[skip : skip + step.get("count", 1)]]
                print(f"{'ok ' if len(picked) == step.get('count', 1) else ('WARN' if step.get('soft') else 'ERR')} clickVisible {step['clickVisible']!r} -> {picked}")
                if len(picked) < step.get("count", 1):
                    (warnings if step.get("soft") else failures).append(f"clickVisible {step['clickVisible']!r}: only {picked} fully on screen: {json.dumps(reply)[:300]}")
                for wid in picked:
                    params = {"id": wid, **({"button": step["button"]} if "button" in step else {})}
                    r = request(sock, reader, "ui.clickWidget", params)
                    log.write(json.dumps({"request": {"method": "ui.clickWidget", "params": params}, "reply": r}) + "\n")
                    time.sleep(step.get("pause", 1))
                continue
            if "shot" in step:
                path = f"{out_dir}/{step['shot']}.png"
                subprocess.run(["xcrun", "simctl", "io", "booted", "screenshot", path], check=True)
                print(f"screenshot {path}")
                continue
            reply = request(sock, reader, step["method"], step.get("params", {}))
            log.write(json.dumps({"request": step, "reply": reply}) + "\n")
            ok = reply.get("ok", True) is not False and "error" not in reply
            print(f"{'ok ' if ok else 'ERR'} {step['method']} {json.dumps(step.get('params', {}))[:120]}")
            if not ok:
                print(f"    {json.dumps(reply)[:400]}")
            for name, path in step.get("capture", {}).items():
                # {"capture": {"rawId": "result.imported.0"}}: later params can use "$rawId"
                names[name] = dig(reply, path)
                print(f"    captured {name} = {json.dumps(names[name])[:80]}")
                if names[name] is None:
                    failures.append(f"{step['method']}: nothing at {path} to capture as {name}: {json.dumps(reply)[:300]}")
            if step.get("expectOk") and not ok:
                failures.append(f"{step['method']} {json.dumps(step.get('params', {}))[:120]} failed: {json.dumps(reply)[:300]}")
            want = step.get("expectPhotosAtLeast")
            if want is not None:
                got = (reply.get("result") or {}).get("photos", 0) if ok else 0
                if got < want:
                    failures.append(f"{step['method']}: expected at least {want} photos, got {json.dumps(reply)[:300]}")
    time.sleep(3)
    if not app_running():
        failures.append("RawPad is no longer running at the end of the tour (crash?)")
    for w in warnings:
        print(f"WARN (not failing): {w}")
    for f in failures:
        print(f"FAIL: {f}")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
