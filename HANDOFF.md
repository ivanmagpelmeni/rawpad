# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-02, about 00:35 Berlin, by the scheduled session (23:55–00:35 Berlin,
  the sixth run of the night).
- **Upstream:** not re-checked this session (checked at 19:54 Berlin: newest tag `v0.9.1`).
- **Main:** run 86 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41358103103), the
  `p6-preview-cap` merge + docs: green, **released 1.86**. Its `memory.jsonl`: `previewSources`
  count ≤ 3 at every step (1 after the memory warning). Nothing new merged into main since.
- **Done this session:** branch **`p6-gpu-pool`** (2 commits, not merged yet):
  - 55bc82fd (DECISIONS 76): `memtrim::set_ipad_limits` settles `memory::budget()` and calls
    `gpu::set_pool_limit(64 MB)` (engine default: budget/8 = 192 MB; run 85 had 177 MB pooled
    while zoomed into the 24 MP raw). Unit test `the_ipad_pools_at_most_64_mb_of_gpu_buffers`.
    `sim_test.py`: new memory-step key `maxGpuPooledMB` (always only a warning for now) on
    `raw-zoomed` and `after-raw`. Finding: leaving the zoomed view already frees the whole pool
    (upstream's `trim_when_idle`), so the pool only mattered for the zoomed peak.
  - 9f702491 (DECISIONS 77): upstream's `lightcraft_sysmem` returns free malloc pages only on
    macOS, and the iOS host never set the engine's release hook, so `memory::release` did nothing
    on the iPad. `native::release_free_memory` (one FFI call to `malloc_zone_pressure_relief`,
    `// SAFETY:` note) + `memory::set_release_hook` in `ios.rs` › `start`; the memory warning logs
    "memory warning: returned X MB of free pages" (tour: soft `checkLog`). Aim: the resident size
    that rose from ~470 to ~547 MB after zooming out (runs 83–86) should fall.
  - DECISIONS 76 and 77 are written on the branch only (they reach main with the merge).
  - Branch run 88 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41359954963): unit
    tests (incl. the new one) and the simulator build green; the whole simulator tour passed (23:32 UTC, then the device build); its report (`memory.jsonl`, screenshots) was not published yet when the session ended
- **Very next step:**
  1. Check branch run 88: green? Then read `runs/88/memory.jsonl` (`ci-artifacts`): `raw-zoomed`
     gpu `pooled` ≤ 64 MB and `allocated` lower than 489 MB; `after-raw` `rssMB` lower than
     ~547; `tour.log` has `ok checkLog 'memory warning: returned'`. Look at shot
     `13-raw-zoomed` (zoomed raw still drawn). If all good, merge into main
     (`git fetch origin; git checkout main; git merge --ff-only origin/main;
     git merge --no-ff origin/p6-gpu-pool`) and push — only when no main run is going.
  2. If the pool check passed, make `maxGpuPooledMB` hard (and the `returned` checkLog) in a
     follow-up with a DECISIONS entry.
  3. Then PROGRESS.md › Next 3 tasks › 2 (touch polish: small touch targets) or the upstream
     merge at the first run of the night.
- **Open problems:**
  - Whether a 64 MB pool slows panning a zoomed raw is not measured (the tour has no pan timing
    on the raw). Watch `ui.inspect` › `perf` if Ivan reports slow pans.
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Add to Apple Photos in the simulator stays flaky (soft check).
  - `apps/rawpad-ios/Cargo.toml` still lists `lightcraft-sysmem` (now unused on iOS); removing it
    changes Cargo.lock, which this sandbox can't regenerate: leave it.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`,
    `p6-hard-checks`, `p6-preview-cap`.
  - The artifact zip download URL (blob.core.windows.net) is blocked from the sandbox: wait for
    the `ci-artifacts` report.
  - `ui.inspect`'s `window` is [820, 1110] pt in portrait on the CI simulator; the `simctl`
    screenshot stays in the portrait framebuffer, so landscape shots look turned sideways.
  - `ci-artifacts` `LATEST` can name a cancelled run's number: always list `runs/` and pick the
    folder yourself.
  - Main runs take longer than branch runs (about 40 minutes with the release): don't push code
    to main while one is going (it cancels it).
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
    `git reset --hard` (and a chained fetch/checkout/reset) is refused in the sandbox: use
    `git checkout main` then `git merge --ff-only origin/main`.
    `git fetch origin main` overwrites FETCH_HEAD: fetch `ci-artifacts` again before `git show`.
  - A memory step's numbers: `runs/<n>/memory.jsonl` (one JSON line per `memory` tour step).
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once, but a run triggered on a branch that already has one queued
    replaces it (concurrency group per ref). A push to a branch doesn't start a run, so push every commit
    before triggering. CI only runs the `tests_curve` module of the UI crate's unit tests, so
    app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. Panels: `panel.edit`, `panel.presets` (toggle!), `panel.versions`, `panel.activity`;
    to open a panel for sure use `ui.set {"right": "edit"}` (also `openSections`).
  - `crate::i18n::tr_format!` accepts only messages already in the translations; build new text
    from existing messages or plain `tr`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time). Shell commands time out
    after 2 minutes: wait for CI with `sleep 115` per call.
