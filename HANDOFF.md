# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-25, about 23:20 Berlin, by the scheduled session (22:55–23:20 Berlin).
- **Upstream:** not re-checked (not the first run of the night; the newest tag was `v0.9.1` at 20:35).
- **Main:** run 57 green and **released 1.57** (Select Several strip + Sync Settings)
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40236028354). Then this session merged
  **`p4-reset-menu`** (branch run 56 green, `a_sliders_name_menu_resets_it ... ok`); main's
  **run 60** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40238860698) was going at
  handoff and should release about 1.60.
- **Done this session:**
  - Sync Settings toast (open problem from run 54): the toast lasts only 1.4 s (`TOAST_SECONDS`)
    and the tour waited 1 s plus the screenshot's own delay, so it was probably just gone. The
    tour now takes `10-select-several-synced` 0.3 s after the tap, and a new app-level test
    `tests_curve::the_select_several_strips_sync_copies_settings_to_the_other_photos` sets
    exposure 0.75 on photo 1, selects 1+2, clicks `button:selectSeveralSync` and checks that
    photo 2's exposure is 0.75 and the toast starts with "Sync Settings". To run it on the
    macOS host, `grid::touch_multi` is now also on under `cfg!(test)` (only when a test sets the
    flag).
  - New branch **`p4-preset-preview`** (contains `p4-sync-test`): on iOS (and in unit tests) a
    preset's long-press menu previews the preset in the loupe while it is open, and its first
    item is **Apply** (`presetMenu:apply:<id>`). DECISIONS 62, UPSTREAM rows. Test
    `tests_curve::a_presets_long_press_menu_previews_and_applies_it` **passed** in run 59.
  - Runs 58/59 failed only because the sync test asked for view `"grid"` (the name is
    `"photoGrid"`); fixed in `cbfbac2`. **Run 61 on `p4-preset-preview`**
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40239152026): its UI unit tests step
    **passed** (both new tests); the simulator tour was still going at handoff.
- **Very next step:**
  1. Check main's run 60 is green and released (`…/releases?per_page=2`). If red, fix or revert
     `f0121c0` (the reset-menu merge) first.
  2. Run 61: `runs/61/ui-tests.log` must show both new tests `ok` (13 passed); look at
     `runs/61/10-select-several-synced.jpg` for the "Sync Settings: 2 photos" toast at the bottom
     of the grid. If green, merge `origin/p4-preset-preview` into main with a merge commit (it
     contains `p4-sync-test`; keep main's `sidestore/source.json`; docs conflicts: keep both).
     **Don't push code to main while main's run is going** (it cancels the release).
  3. Optional: tour steps for the new menus (Develop: `ui.clickWidget` with `"button":"right"` on
     `sliderLabel:light.exposure`, shot, click `sliderMenu:reset:light.exposure`; Presets:
     `panel.presets`, right-click a `preset:` row, shot with the preview, click
     `presetMenu:apply:<id>`).
  4. Then PROGRESS.md › Next 3 tasks (snapshot preview by long-press is next).
- **Open problems:**
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - The filmstrip in the loupe has no Select Several indicator (only the menu's *Done Selecting*).
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu` (and `p4-sync-test`, `p4-preset-preview` once merged).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `ui-tests.log`, screenshots).
    `LATEST` is the last run to publish, not the newest number: look at `runs/<n>/` directly.
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
    CI pushes `SideStore source: RawPad x.y` to main after each release: fetch before merging.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once. CI only runs the `tests_curve` module of the UI crate's unit
    tests, so app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time).
