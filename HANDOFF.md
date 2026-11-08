# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-08, about 22:30 Berlin, by the scheduled session (21:54–22:35 Berlin).
- **Upstream:** **v0.19.0** (2026-11-06) is the newest tag; main is still on v0.9.1. The merge
  lives in branch `upstream-0.19.0` (DECISIONS 83–89).
- **Main:** green, untouched (last code run 96, release 1.96).
- **Done this session (branch `upstream-0.19.0`):**
  - **Cause of run 102's 18 failed bottom-bar taps:** the safe-area insets came off twice. The
    host shrinks `raw.screen_rect` to UIKit's safe area; egui-winit 0.37 now also reports the
    insets (`safe_area_insets`, kept by egui), and egui's background layer is
    `viewport_rect - safe_area_insets`, so it ended ~20 pt above the screen bottom and
    `ctx.layer_id_at` found no layer under the bar (`control::widget_target_visible` → false).
    d5fc483d: the host sends zero insets with its own rect (`ios.rs` › `frame`, DECISIONS 89).
    Probably real touches on the bar were dead too on 0.19.
  - UPSTREAM.md's patch table now names the 0.19.0 files row by row (+ a row for the updater
    switched off on iOS).
  - **Branch run 103** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42522720374) on
    d5fc483d: fmt, both unit-test steps, simulator build, clippy **and the tour step green**
    (run 102's tour step was red). At writing, the device build and the report were still
    running: **look at `runs/103/tour.log` (grep ERR) and `runs/103/08-*.jpg` first.**
- **Very next step:**
  1. `git fetch --depth 1 origin ci-artifacts`; check `runs/103/tour.log` has no `ERR`/`FAIL`
     and that `08-split.jpg`, `08-rating-menu.jpg`, `08-label-menu.jpg` show the menus/views.
     Check the run's overall conclusion (device build) too.
  2. If clean: merge into main **at the start of the session** (`git checkout main; git merge
     --ff-only origin/main; git merge --no-ff origin/upstream-0.19.0`), push, watch the main
     run to the end (about 40 minutes with the release). Update UPSTREAM.md's "Last merged"
     line and merge log, and the release number in PROGRESS.md › Needs Ivan (what to test).
  3. If not clean: fix on the branch, trigger `ipad.yml` with `{"release": false}`, repeat.
- **Open problems:**
  - Returning free pages (DECISIONS 77) freed 0 MB in the simulator; only a real iPad's log
    ("memory warning: returned X MB") would tell. Resident size after zooming out stays ~550 MB.
  - Whether a 64 MB pool slows panning a zoomed raw is not measured. Watch `ui.inspect` › `perf`
    if Ivan reports slow pans.
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Add to Apple Photos in the simulator stays flaky (soft check).
  - `apps/rawpad-ios/Cargo.toml` still lists `lightcraft-sysmem` (now unused on iOS); removing it
    changes Cargo.lock, which this sandbox can't regenerate: leave it.
  - The upstream merge took ten releases at once; upstream tests that measure the bottom-bar pill
    (`bottombar.rs` tests) may fail under `cfg(test)` because the label button is on in tests.
    CI doesn't run them; fix if they get in the way.
  - Merged branches to delete on GitHub some time: `p7-label-pill`, `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`,
    `p6-hard-checks`, `p6-preview-cap`, `p6-gpu-pool`, `p7-stroke-start`.
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
    before triggering. CI only runs the `tests::ipad` module of the UI crate's unit tests
    (`crates/ui-egui/src/tests/ipad.rs`), so app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. Panels: `panel.edit`, `panel.presets` (toggle!), `panel.versions`, `panel.activity`;
    to open a panel for sure use `ui.set {"right": "edit"}` (also `openSections`).
    `ui.resize {"width", "height"}` resizes the headless window in unit tests.
  - `crate::i18n::tr_format!` accepts only messages already in the translations; build new text
    from existing messages or plain `tr`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time). Shell commands time out
    after 2 minutes: wait for CI with `sleep 115` per call.
