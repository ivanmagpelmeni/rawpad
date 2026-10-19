# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-19, about 03:30 Berlin, end of nightly session 9 (unattended, started 02:55;
  the last run of the night).
- **Upstream:** not checked (not the first run of the night).
- **Done this session:**
  - Main run 35 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39181393085) green,
    **released 1.35** (UIKit hardware keys). `04-final` screenshot looked at.
  - Its log: osascript's `5` arrives as HID 0x5D (keypad 5, flags 0x200000) and the HID table
    dropped it, so the rating never changed and the ⌘Z check passed vacuously. Keypad digits now
    map to Num1…Num0 (DECISIONS.md 44).
  - **On-screen keyboard:** `native::TextKeyView` (adopts `UIKeyInput`) takes first responder
    while `ctx.egui_wants_keyboard_input()`; `KeyView` otherwise (DECISIONS.md 43).
    `keys::soft_text_events` / `soft_backspace` (unit-tested). Tour clicks `field:search`, shoots
    `04-text-field`, presses Escape.
  - Branch run 37 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39183596821) **all
    green**: `5` → rating 5 `ok`, ⌘Z → rating 3 `ok` (first real proof of hardware keys + undo);
    `app-stderr.log` shows `first responder for text field (on-screen keyboard) taken`, then
    `for keys taken` after Escape; `runs/37/04-text-field.jpg` shows the iPad keyboard under the
    focused search box (the simulator showed it despite the hardware-keyboard probe).
  - **Merged `p4-soft-keyboard` into main** (`082ed01`) and pushed at about 03:25 (with the CI's 1.35 source update merged in, `2eae72e`): main
    run 38 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39185203734) was running at
    handoff: build, clippy and the simulator tour already passed (03:38), only the .ipa build and
    release were left; it should release 1.38.
- **Very next step:**
  1. Check main run 38 (id 39185203734): green and released? If red, fix or revert the merge
     commit first (`git revert -m 1 082ed01`).
  2. If its two keyboard `waitUntil rating` steps say `ok` again, remove `"soft"` from them in
     `ios/sim_tour.jsonl`.
  3. Then PROGRESS.md › Next 3 tasks #2: keyboard avoidance (the on-screen keyboard covers the
     bottom of the UI; egui's screen rect should shrink by the keyboard's height).
- **Open problems:**
  - The on-screen keyboard covers the lower part of the screen; fields there are hidden behind it.
  - Typing on the on-screen keyboard isn't exercised in CI yet (only that it appears).
  - Simulator Photos flakiness is only tolerated (soft step), not explained.
  - "Pen Pressure" is ticked by default; the USB-C Pencil has no pressure (asked Ivan).
  - Real-finger pinch/pan, long-press, Pencil, keyboard, Photos permission and the share sheet
    are untested on the real iPad (Needs Ivan).
  - Merged branches (`p5-*`, `p4-*`) not deleted.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `build-sim.log`, screenshots).
    raw.githubusercontent.com is blocked from this sandbox; use git.
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `actions_run_trigger`, `ipad.yml`, ref = branch, `{"release": false}`. They don't
    cancel main runs. A full run takes 20–30 minutes; the tour result is known after about 15.
