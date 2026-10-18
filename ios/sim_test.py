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


def main():
    tour, out_dir = sys.argv[1], sys.argv[2]
    port = int(sys.argv[3]) if len(sys.argv) > 3 else 7980
    sock = connect(port)
    reader = sock.makefile("r", encoding="utf-8")
    failures = []
    with open(f"{out_dir}/replies.jsonl", "w") as log:
        for raw in open(tour):
            raw = raw.strip()
            if not raw or raw.startswith("#"):
                continue
            step = json.loads(raw)
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
                    if value or time.time() > deadline:
                        break
                    time.sleep(2)
                log.write(json.dumps({"request": step, "reply": reply}) + "\n")
                print(f"{'ok ' if value else 'ERR'} waitUntil {step['waitUntil']} = {json.dumps(value)[:200]}")
                if not value:
                    failures.append(f"{step['method']}: {step['waitUntil']} never became true: {json.dumps(reply)[:300]}")
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
    for f in failures:
        print(f"FAIL: {f}")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
