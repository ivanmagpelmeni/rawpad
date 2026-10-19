# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-19, about 02:35 Berlin, end of nightly session 8 (unattended, started 01:54).
- **Upstream:** not checked (not the first run of the night).
- **Done this session:**
  - Main run 32 (Pencil + `p4-keys`) green, **released 1.32**. `runs/32/03-brush.jpg` looked at.
  - Run 30 had **no** winit `keyboard:` lines (and its probe ran under the share sheet). So
    `p4-uikit-keys`: `native::KeyView`, a 1×1 input-transparent UIView in winit's view that holds
    first responder (not while a dialog is presented) and turns `pressesBegan/Ended` into egui
    events (`keys::uikit_events`, HID table, unit-tested). DECISIONS.md 41, 42.
  - Branch run 33 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39178764596) green; its
    `app-stderr.log` shows `keyboard (UIKit): key 0x13 … "p" down`, ⌘ (0xE3) and Z with
    flags 0x100000. ⌘Z did not undo the exposure step, but `p` (pick flag, already set) came first,
    so ⌘Z probably undid that: inconclusive. The log showed ⌘'s release still carrying the ⌘ flag
    → fixed (`uikit_flags_after`) in `2b8bd92`.
  - Tour probe rewritten: select photo 1 (3 stars), host key `5` → `waitUntil rating equals 5`,
    ⌘Z → `equals 3`, both `"soft"`. `sim_test.py` gained `"equals"` for `waitUntil`.
  - Branch run 34 (`2b8bd92`) passed unit tests, build and clippy; its tour was still running.
    **Merged `p4-uikit-keys` into main** (`f489485`, plus the 1.32 source update `a06e6ae`): main
    run 35 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39181393085) was running at
    handoff and should release about 1.35.
- **Very next step:**
  1. Check the main run after the merge (run 35, id 39181393085): green and released? If red, fix or revert
     the merge commit first.
  2. In its `tour.log`: the two `waitUntil rating` lines (`ok` or `WARN`). Both ok → remove
     `"soft"`. `5` ok but ⌘Z WARN → the Simulator app's own Edit menu may take ⌘Z; try another ⌘
     shortcut in the probe (check `app-stderr.log` for a `0x1D flags 0x100000` line first).
  3. Then PROGRESS.md › Next 3 tasks #2: the on-screen keyboard (`UIKeyInput` on `KeyView`).
- **Open problems:**
  - Simulator Photos flakiness is only tolerated (soft step), not explained.
  - "Pen Pressure" is ticked by default; the USB-C Pencil has no pressure (asked Ivan).
  - Text fields probably can't open the on-screen keyboard (winit 0.30 iOS has no IME); not
    verified yet. With `KeyView` as first responder, adding `UIKeyInput` there is the plan.
  - Real-finger pinch/pan, long-press, Pencil, keyboard, Photos permission and the share sheet
    are untested on the real iPad (Needs Ivan).
  - Merged branches (`p5-*`, `p4-pinch`, `p4-top-inset`, `p4-batch`, `p4-pencil`, `p4-keys`,
    `p4-uikit-keys`) not deleted.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `build-sim.log`, screenshots).
    raw.githubusercontent.com is blocked from this sandbox; use git.
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `actions_run_trigger`, `ipad.yml`, ref = branch, `{"release": false}`. They don't
    cancel main runs. A full run takes 20–30 minutes; the tour result is known after about 15.
