# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-08, about 21:32 Berlin, by the scheduled session (20:55–21:35 Berlin).
- **Upstream:** **v0.19.0** (2026-11-06) is the newest tag; main is still on v0.9.1. The merge
  lives in branch `upstream-0.19.0` (DECISIONS 83–88).
- **Main:** green, untouched (last code run 96, release 1.96).
- **Done this session (branch `upstream-0.19.0`):**
  - Run 100 (previous session's handoff commit) failed one unit test,
    `memtrim::the_ipad_pools_at_most_64_mb_of_gpu_buffers` (165 MB pooled): a race between
    parallel tests over the lazily computed memory budget. Fixed in bb1bbd34 (DECISIONS 88).
  - **Cause of run 99's 24 failed tour taps:** upstream 0.19's first-run *Welcome to
    LightCraft* modal covered the app (`runs/99/08-before-after-menu.jpg`,
    `10-select-several-menu.jpg`). 81657d61: no Welcome / What's New on iOS
    (`startup::open_at_startup`, DECISIONS 87, UPSTREAM.md row).
  - `button:rawpadShareExport` was listed by `ui.widgets` but `ui.clickWidget` said "no widget":
    upstream now publishes `app.widgets` from a registry at the end of each pass, so the host's
    push after `app.ui` was gone when the click was resolved. 0981ebed: the host registers the
    button with `widgets::register_response` (needs `Shown.id`; `ios.rs`, `share_offer.rs`).
  - **Branch run 102** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42517809756) on
    0981ebed: fmt, both unit-test steps, simulator build and clippy **green**; the tour was
    still running when this session ended (it ran > 10 minutes, longer than run 99's, which
    suggests the taps now go through). Its report lands in `runs/102/` on `ci-artifacts`.
- **Very next step:**
  1. Read run 102's result and `runs/102/tour.log` (`grep -n ERR`), then look at the 08-*, 10-*,
     11-* and 05-share-offer / 06-share-sheet screenshots.
  2. If 0 ERR and the shots look right: merge into main **at the start of a session**
     (`git checkout main; git merge --ff-only origin/main; git merge --no-ff
     origin/upstream-0.19.0`), push, watch the main run to the end (about 40 minutes with the
     release). Then tell Ivan in PROGRESS (Needs Ivan) that the next SideStore update is the big
     upstream 0.19 one.
  3. If taps still fail: the press-time check is `control::widget_target_visible`
     (`crates/ui-egui/src/control.rs` ~309): a target needs a registered egui hit (`register`
     or `register_response`) on the top layer at the point. Fix, push, trigger `ipad.yml` on
     the branch with `{"release": false}`, repeat.
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
