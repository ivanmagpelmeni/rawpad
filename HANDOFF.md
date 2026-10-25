# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-25, about 22:38 Berlin, by the scheduled session (21:55–22:40).
- **Upstream:** not re-checked (not the first run of the night; the newest tag was `v0.9.1` at 20:35).
- **Main:** run 53 for `ca54af2` was green and **released 1.53**
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40231250240). Then this session merged
  **`p4-select-bar` + `p4-select-sync`** (merge commit `b31c5e9`, pushed about 22:38 with these
  docs; `sidestore/source.json` kept at main's 1.53). Main's run 57 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40236028354) was not finished at
  handoff; it should release about 1.57.
- **Done this session:**
  - Branch run 52 on `p4-select-bar` green
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40231165148): tour log
    `clickVisible 'thumb:' -> ['thumb:15', 'thumb:8']`, `waitUntil selected = true`; screenshots
    `10-select-several` (strip, 3 highlighted, "3 selected") and `10-select-several-done` (strip
    gone) checked.
  - Branch **`p4-select-sync`** (from `p4-select-bar`): **Sync Settings** button in the strip,
    left of Done (`button:selectSeveralSync`), greyed out until 2 photos are selected; runs
    `develop.sync`; toast "Sync Settings: N photos" (reuses translated messages). DECISIONS 60,
    UPSTREAM row updated, tour step + shot `10-select-several-synced`. Run 54 green
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40233169370); the screenshot shows the
    button, but **no toast was visible 1 s after the tap** (see open problems).
  - Branch **`p4-reset-menu`** (from `p4-select-sync`): on iOS (and `cfg!(test)`) a long-press on a
    slider's name opens a menu with **Reset <name>** (`sliderMenu:reset:<control>`), in
    `widgets::slider` (`menu_reset`). DECISIONS 61, UPSTREAM row. Run 55
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40233272175) failed only in a unit test
    that drove raw egui frames (it probably clicked the menu item at its first-frame "sizing pass"
    position); replaced by the app-level test `tests_curve::a_sliders_name_menu_resets_it`
    (right-click via `ui.clickWidget`, then click the item), commit after `e91f502`/`3341447`.
    **Run 56 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40234543833) dispatched at about 22:14 Berlin**, not finished at handoff.
- **Very next step:**
  1. Check main's run for `b31c5e9` (and the docs commit after it) is green and released
     (`…/releases?per_page=2`). If red, fix or revert the merge first.
  2. Run 56 on `p4-reset-menu`: `runs/56/ui-tests.log` must show
     `a_sliders_name_menu_resets_it ... ok`. If green, merge `origin/p4-reset-menu` into main with
     a merge commit (it contains `p4-select-sync`, already merged; conflicts only in docs: keep
     both sides; keep main's `sidestore/source.json`).
  3. Optional: add a tour step for the slider menu (right-click `sliderLabel:light.exposure` with
     `"button":"right"` in Develop, shot, click `sliderMenu:reset:light.exposure`).
  4. Then PROGRESS.md › Next 3 tasks.
- **Open problems:**
  - The Sync Settings toast didn't show in run 54's `10-select-several-synced.jpg`. Either the
    toast is drawn elsewhere / gone after 1 s, or `develop.sync` errored silently. Check
    `runs/54/replies.jsonl` around the `button:selectSeveralSync` click and `app-stderr.log`; a
    tour check that photo 2's exposure follows photo 1's after the sync would prove it.
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - The filmstrip in the loupe has no Select Several indicator (only the menu's *Done Selecting*).
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`.
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
    two branch runs can go at once.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time).
