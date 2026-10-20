# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-20, about 09:17 Berlin, end of a morning session (unattended, started
  08:40).
- **Upstream:** not checked (not the first run of the night).
- **Done this session:**
  - Main run 38 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39185203734) was green and
    **released 1.38** (on-screen keyboard). Both keyboard rating checks said `ok` again, so they
    are hard checks now (DECISIONS.md 46).
  - **Keyboard avoidance** (`src/keyboard.rs`, `native::keyboard_cover`, DECISIONS.md 45): the
    docked keyboard's height comes from `keyboardLayoutGuide`; while it covers the focused text
    field the egui screen rect shrinks by it. Decided once per field (no layout jumping),
    6 unit tests.
  - Tour: types `rq` into the search box, ⌘A, copy, checks `ui.inspect` → `copied` = "rq"
    (soft; it passed: typing arrives once), then opens the Info panel and focuses the caption
    with the keyboard up (`04-caption-keyboard`).
  - Branch run 39 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39382129144) passed
    build, unit tests, clippy, the tour and the .ipa build. Log: `on-screen keyboard covers 257
    pt, UI shrunk by 0 pt` for both the search box and the caption: correct, both sit above the
    keyboard (screenshot `runs/39/04-caption-keyboard.jpg` looked at).
  - Merged `p4-keyboard-avoid` into main and pushed at about 09:17; the main run after that push
    should release about 1.40.
- **Very next step:**
  1. Check that main run (the first after `3681a8a`): green and released? If red, fix or
     `git revert -m 1 3681a8a`.
  2. The **shrinking path itself is not exercised in CI yet** (both fields were above the
     keyboard). Add a tour step with a field low on the screen: e.g. in the Info panel scroll
     down (`ui.scroll` over the right panel) and click `field:gps` or a field near the bottom,
     then expect `UI shrunk by` > 0 in `app-stderr.log` and look at the screenshot.
  3. Check the keyboard height: the guide said 257 pt above a 25 pt safe area, but the keyboard
     in the screenshot looks about 300+ pt tall. If a field just above the keyboard still ends
     up covered, the guide's frame may need converting (`convertRect:toView:`) or `MARGIN`
     raising.
- **Open problems:**
  - Keyboard height may be under-read (see step 3); the shrink path is only unit-tested.
  - A field inside a scroll area that ends up below the shrunk panel's bottom edge is only
    scrolled into view when typing starts (egui's TextEdit scrolls to its cursor).
  - Simulator Photos flakiness is only tolerated (soft step), not explained.
  - "Pen Pressure" is ticked by default; the USB-C Pencil has no pressure (asked Ivan).
  - Nothing tested on the real iPad yet (Needs Ivan).
  - Merged branches (`p5-*`, `p4-*`) not deleted.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `build-sim.log`, screenshots).
    raw.githubusercontent.com is blocked from this sandbox; use git. The report is published
    only after the .ipa build (about 25 minutes into a run).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `actions_run_trigger`, `ipad.yml`, ref = branch, `{"release": false}`. They don't
    cancel main runs. A full run takes 25–30 minutes; the tour result is known after about 17.
