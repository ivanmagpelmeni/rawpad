# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-02, about 01:40 Berlin, by the scheduled session (00:55–01:40 Berlin,
  the seventh run of the night).
- **Upstream:** not checked: `storytold/lightcraft` isn't attached to this session (the public
  API answers "use add_repo"). Last known: `v0.9.1` (checked 2026-11-01 19:54 Berlin).
- **Main:** `p6-gpu-pool` merged (b862fa38) plus 9d77bcba (DECISIONS 78: the tour's
  `maxGpuPooledMB` is hard now). Main run 89
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41364478335) started 23:55 UTC and
  releases about 1.89 if green: still in the device build at 00:33 UTC (tour, unit tests, clippy all passed; GPU pool check hard and passing).
- **Done this session:**
  - Branch run 88's report checked: `raw-zoomed` GPU pooled 54.7 MB (≤ 64; run 85: 177 MB),
    allocated 390 MB (was 512); shot `13-raw-zoomed` draws the zoomed raw; `after-raw` rss 564 MB
    (no better than 547); "memory warning: returned 0.0 MB of free pages" in the simulator. So
    the pool cap works, the page release shows no effect in the simulator (kept, soft check).
  - Branch **`p7-stroke-start`** (ae432403, DECISIONS 79, UPSTREAM row): on iOS and in unit
    tests, a brush or Remove stroke that starts with a drag begins at `press_origin()` (audit
    item 15: the first few millimetres were lost). `detail.rs › stroke_lead_in`, unit test
    `tests_curve::a_brush_or_spot_stroke_starts_where_the_finger_touched`.
    Branch run 90 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41364616737): unit tests (incl. the new one), clippy and the whole tour green (0 ERR; GPU pool 52 MB), report `runs/90/` published; shot `03-brush` paints the mask. **Not merged** only because main run 89 was still going.
- **Very next step:**
  1. Main run 89 green? (If red: read its log; fix or revert 9d77bcba / the merge.)
  2. Branch run 90 was green (check it finished `success`). Then, only when no main run is going: `git fetch origin; git checkout
     main; git merge --ff-only origin/main; git merge --no-ff origin/p7-stroke-start`, push.
  3. Then PROGRESS.md › Next 3 tasks › 2 (hover-only reset hints, colour labels in the bottom bar).
- **Open problems:**
  - Returning free pages (DECISIONS 77) freed 0 MB in the simulator; only a real iPad's log
    ("memory warning: returned X MB") would tell. Resident size after zooming out stays ~550 MB.
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
    `p6-hard-checks`, `p6-preview-cap`, `p6-gpu-pool`.
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
