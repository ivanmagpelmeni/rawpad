# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-25, about 21:35 Berlin, by the scheduled session (20:54–21:40).
- **Upstream:** not re-checked (not the first run of the night; at 20:35 the newest tag was still `v0.9.1`).
- **Main:** **`p4-select-several` merged** (merge commit `ca54af2`, pushed about 21:35 with these
  docs). Its branch run 50 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40228483392)
  was fully green; screenshot `runs/50/10-select-several.jpg` shows 3 highlighted photos,
  "3 selected · 24 photos" and Quick Develop · 3 photos. **Main's run 53 for the merge
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40231250240) was still going at handoff** and should release about 1.50 (radial knob, star touch area, targeted
  adjustment at the press, Select Several, colour-wheel eye always shown).
- **Done this session:**
  - Run 49's failure was in the tour: `thumb:2` was laid out above the screen (y = −509) and
    `thumb:3` not at all. New runner step `clickVisible` (`ios/sim_test.py`) asks `ui.widgets`
    and taps the first N matching widgets fully between `top` and `bottom` (defaults 50/1130).
  - Run 48 attempt 2 green → its last-step failure (app drew no frame for 5 s with the share
    sheet over it) was a flake. The Add-to-Photos request after the share sheet is no longer
    `expectOk` (tour comment says why).
  - Branch **`p4-select-bar`** (from `p4-select-several`): a 40 pt strip under the grid title
    while Select Several is on, "Tap photos to add or remove them" + **Done**
    (`button:selectSeveralDone`), DECISIONS 59, UPSTREAM.md row. Run 51
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40228625405): built, clippy and tests
    green, strip looks right in `runs/51/10-select-several.jpg`, but the tour picked `thumb:22`
    whose rect started at y = 79, under the strip; the tap hit the day header (which selects that
    day), so 2 selected instead of 3. Fixed by `"top":160,"bottom":1100` on that tour step
    (commit after `9edf9cd`). **Run 52 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40231165148) dispatched at about 21:31 Berlin**, not finished.
- **Very next step:**
  1. Check main's run for `ca54af2` is green and released (`…/releases?per_page=2`). If red,
     fix or revert the merge first.
  2. Run 52 on `p4-select-bar`: tour log `runs/52/tour.log` should show
     `clickVisible 'thumb:' -> [two ids]` and `ok waitUntil selected = true`; look at
     `10-select-several.jpg` (strip + 3 highlighted) and `10-select-several-done.jpg` (strip
     gone). Then merge `origin/p4-select-bar` into main with a merge commit (conflicts possible
     only in DECISIONS/UPSTREAM/HANDOFF/PROGRESS: keep both sides, DECISIONS 59 is the strip).
  3. Then PROGRESS.md › Next 3 tasks (a Sync… button in the strip; hover-only leftovers).
- **Open problems:**
  - `ui.inspect` › `canvasRect` in the grid is `ui.max_rect()`, which includes the title row, so
    the runner can't use it as the visible area; that's why `top` is hard-coded in the tour.
  - The filmstrip in the loupe has no Select Several indicator (only the menu's *Done Selecting*).
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several` (and `p4-select-bar` once merged).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `ui-tests.log`, screenshots).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time).
