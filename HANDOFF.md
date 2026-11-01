# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-01, about 20:30 Berlin, by the scheduled session (19:54–20:40 Berlin,
  the second run of the night).
- **Upstream:** checked (`git ls-remote --tags`): newest tag still `v0.9.1`, nothing to merge.
- **Main:** run 74 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41336426686) green,
  **released 1.74** (rotation tour, hard checks passed: `ok hostMenu`, `ok waitUntil window`).
  Main is unchanged by code this session (docs only).
- **Done this session (DECISIONS 70, 71), on two branches, NOT merged yet:**
  - **`p4-raw24`** (commit 734ee6f9): the 24 MP raw memory test. CI downloads the CC0 Sony A7 III
    ARW (sha-checked) into the app's `tmp/`; the tour imports it, opens, edits, zooms 4× and logs
    memory to `memory.jsonl` (shots `13-raw-loupe`, `13-raw-edited`, `13-raw-zoomed`). New
    `sim_test.py` features: `ifFile`, `capture`, `$name`/`$APPDATA` substitution, `memory` steps.
    **Branch run 75** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41339239175): build,
    tests and tour green; **merged into main** (its main run releases about 1.76). Checked the
    `13-raw-*` shots: the A7 III photo opens, edits (+0.70 exposure, +40 shadows) and zooms to
    60 % with the navigator. `memory.jsonl` (resident MB / engine caches MB): before 184 / 584,
    open 285 / 586, edited 291 / 591, zoomed 436 / 612, back on photo 1 557 / 591. Budget 1536 MB;
    the 24 MP original holds 288 MB. (`ps` resident size is below the engine total: macOS
    compresses idle memory, so treat RSS as a trend only.)
  - **`p6-memtrim`** (commits 92f45ac7, 5a75ec21): memory warnings drop cached photos
    (`apps/rawpad-ios/src/memtrim.rs`, two unit tests) and the tour sends *Debug › Simulate
    Memory Warning* then checks the log line (soft). **Branch run 78**
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41341201671) was still running at the
    end (run 77 was cancelled by GitHub while queued; 76 cancelled by me).
- **Very next step:**
  1. Check main's run for the `p4-raw24` merge is green (releases about 1.76).
  2. Run 78: unit tests `memtrim` passed? In `tour.log`: `ok hostMenu Debug > Simulate Memory
     Warning` and the `checkLog 'memory warning: caches held'` line. If the unit test
     `a_memory_warning_drops_cached_photos_but_keeps_the_budget` fails (after == before), the demo
     renders may not fill the media cache: render more photos or check `held()`. Then merge
     `p6-memtrim` (`p4-raw24` is already in main) (both touch `sim_tour.jsonl` right after `12-portrait-again`:
     expect a trivial conflict; keep both blocks, memory warning first).
  3. Once both are in: make the memory-warning check hard, and consider tightening `maxRssMB`.
- **Open problems:**
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate` (and later `p4-raw24`, `p6-memtrim`).
  - The artifact zip download URL (blob.core.windows.net) is blocked from the sandbox: wait for
    the `ci-artifacts` report.
  - `ui.inspect`'s `window` is [820, 1110] pt in portrait on the CI simulator (1180 minus the top
    gap and safe area); the `simctl` screenshot stays in the portrait framebuffer, so landscape
    shots look turned sideways.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `ui-tests.log`, screenshots).
    `LATEST` is the last run to publish, not the newest number: look at `runs/<n>/` directly.
    The report is published after the device build, about 10 minutes after the tour ends.
    Compile errors of the UI tests are in `ui-tests.log` (not `unit-tests.log`).
    raw.githubusercontent.com is blocked from the sandbox: use `git fetch` as above.
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
    CI pushes `SideStore source: RawPad x.y` to main after each release: fetch before merging.
    `git reset --hard` is refused in the sandbox: use `git checkout -B main origin/main`.
  - A memory step's numbers: `runs/<n>/memory.jsonl` (one JSON line per `memory` tour step).
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once. A push to a branch doesn't start a run, so push every commit
    before triggering. CI only runs the `tests_curve` module of the UI crate's unit tests, so
    app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. Panels: `panel.edit`, `panel.presets` (toggle!), `panel.versions`, `panel.activity`;
    to open a panel for sure use `ui.set {"right": "edit"}` (also `openSections`).
  - `crate::i18n::tr_format!` accepts only messages already in the translations; build new text
    from existing messages or plain `tr`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time).
