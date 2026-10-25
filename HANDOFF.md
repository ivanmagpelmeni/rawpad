# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-26, about 00:38 Berlin, by the scheduled session (23:55–00:40 Berlin).
- **Upstream:** not re-checked (not the first run of the night).
- **Main:** run 60's re-run (attempt 2) was **green and released 1.60** (slider name → Reset):
  the attempt-1 `checkLog` failure was a flake. Then this session **merged `p4-preset-preview`**
  into main (`c186a93`; preset preview by long-press + the Sync Settings test, branch run 61
  green); **main's run 64** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40243221933)
  was going at handoff and releases about 1.64.
- **Done this session (branch `p4-snapshot-preview`, contains `p4-preset-preview`):**
  - `f9e6285`, `1edba48` P4: a **version's** (LightCraft's snapshots are *Versions*) long-press menu
    previews it in the loupe while open (first item *Restore*, id `versionMenu:restore:<i>`), and
    a **History step's** long-press menu previews the photo at that step and starts with *Restore*
    (`historyMenu:restore:<i>`). iOS + `cfg!(test)`. DECISIONS 63, UPSTREAM row. Tests
    `tests_curve::a_versions_long_press_menu_previews_and_restores_it` and
    `tests_curve::a_history_steps_long_press_menu_previews_and_goes_back_to_it` **passed** in
    branch run 63 (UI unit tests step green).
  - Tour: new soft steps for the long-press menus with shots `11-slider-menu`,
    `11-history-preview`, `11-version-preview`, `11-preset-preview`. `ios/sim_test.py`:
    `clickVisible` takes `skip`, `button` ("right" = long-press) and `soft`; `checkLog` (present)
    now polls up to 5 s (`wait`), the cause of run 60's first-attempt flake.
  - Branch **run 63** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40242697351): **green** (all steps). Screenshots checked: `11-preset-preview`
    shows "Preset: High Contrast B&W" in the loupe with *Apply* first; `11-history-preview` shows
    the step's label ("Sync Settings") over the photo as it was then, *Restore* first. But the
    slider step failed softly: `no widget sliderLabel:light.exposure on screen` (the Edit panel
    after `panel.edit` didn't show the Light section's labels, maybe toggled closed or scrolled),
    so exposure stayed +0.75 and `11-version-preview` shows *Bright* as the current version (no
    preview, correctly) with the bright photo.
- **Very next step:**
  1. Check main's run 64 is green and released (`…/releases?per_page=2`). If red, fix or revert
     the merge `c186a93`; main must not stay red.
  2. Fix the tour's slider step: before it, `ui.widgets` with filter `sliderLabel` to see what is
     on screen (maybe `panel.edit` toggles the Edit panel shut when it is already open: use
     `ui.set` or check `ui.inspect`), and make the Light section open. Then the version shot
     should show the bright *Bright* version over the reset photo. When all four `11-*` shots
     look right, drop `"soft":true` from the three `clickVisible` steps.
  3. Branch run 63 is green: once main's run 64 is done, merge `origin/p4-snapshot-preview` into main
     with a merge commit (keep main's `sidestore/source.json`; docs conflicts: keep both).
     **Don't push code to main while main's run is going** (it cancels the release).
  4. Then PROGRESS.md › Next 3 tasks.
- **Open problems:**
  - Run 61's `10-select-several-synced.jpg` shows the strip but no toast at 0.3 s; the unit test
    proves the sync and the toast text, so the shot's timing or toast position is still unclear.
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - The filmstrip in the loupe has no Select Several indicator (only the menu's *Done Selecting*).
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview` (and `p4-snapshot-preview` once merged).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `ui-tests.log`, screenshots).
    `LATEST` is the last run to publish, not the newest number: look at `runs/<n>/` directly.
    The report is published after the device build, about 10 minutes after the tour ends.
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
    CI pushes `SideStore source: RawPad x.y` to main after each release: fetch before merging.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once. A push to a branch doesn't start a run, so push every commit
    before triggering. CI only runs the `tests_curve` module of the UI crate's unit tests, so
    app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. Panels: `panel.edit`, `panel.presets` (toggles), `panel.versions`, `panel.activity`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time).
