# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-08, about 20:40 Berlin, by the scheduled session (19:55–20:40 Berlin).
- **Upstream:** **v0.19.0** (2026-11-06) is the newest tag; main is still on v0.9.1. The merge
  lives in branch `upstream-0.19.0` (DECISIONS 83–85).
- **Main:** green. Main run 96 (a20dd94a, `p7-label-pill`) released **1.96** (not 1.97);
  CI then pushed `SideStore source: RawPad 1.96` (6ae8ec86). Nothing was pushed to main's code
  this session.
- **Done this session (branch `upstream-0.19.0`):**
  - Run 98 had failed in "Build for the simulator" with two things: the desktop `logging.rs`
    (shared into the host via `#[path]`) now uses **chrono**, and `raw_input_hook` takes
    `(&ctx, &mut raw)`. Fixed in 11524b9a (chrono added to the host's iOS dependencies).
  - **Branch run 99** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42512460453) on
    11524b9a: fmt, both unit-test steps, **the simulator build and clippy are green**: upstream
    0.19.0 + the iPad patches compile for iOS. The app launches and the tour ran (128 steps
    ok), but **24 tour steps failed** (report `runs/99/` on `ci-artifacts`; `tour.log`).
  - f857f1a1: craft-update (upstream's
    self-updater) is off on the iPad: Help › Check for Updates is hidden on iOS
    (`menubar.rs` › `host_supports`) and the command returns "RawPad updates through
    SideStore…" (`dialogs/updates_ui.rs` › `check`). Not yet in a CI run (it was pushed after
    run 99 was triggered).
- **The tour failures in run 99** (all `ui.clickWidget`):
  - Bottom bar: `icon:beforeAfterModes`, `icon:beforeAfter`, `icon:ratingMenu`,
    `icon:labelMenu` fail with **"canceled: widget … is no longer visible before its press;
    nothing was retargeted"**. The items of the menus they open (`beforeAfterMode:*`,
    `star:bottom:*`, `bottomLabel:*`) then fail with "no widget on screen" as a consequence.
    The message comes from upstream's new press-time check in `crates/ui-egui/src/app/control.rs`
    (around line 250: `control::widget_target_visible(ctx, latest_hit, target.at)`). Guess:
    the bottom bar's iPad buttons are registered with `register(ui, id, rect)` but egui's hit
    test at that point finds another layer/widget (or nothing) on top, e.g. the pill's child
    `Ui`, or a rect that is off by the safe-area inset. Look at what upstream widgets do to
    pass this check (search `widget_target_visible` and `check_visible`), and at the bottom-bar
    screenshots in `runs/99/`.
  - `gridMenu:selectSeveral`, `button:selectSeveralSync`, `button:selectSeveralDone` (Select
    Several, the grid's long-press menu), `sliderMenu:reset:light.exposure` (slider long-press
    menu) and `button:rawpadShareExport` (the host's Share offer): "no widget on screen". Either
    the menus didn't open (long-press timing/ids changed upstream) or the patches don't register
    these ids any more: check `ui.widgets` replies in `runs/99/replies.jsonl` around those steps.
- **Very next step:**
  1. Trigger `ipad.yml` on `upstream-0.19.0` with `{"release": false}` (GitHub MCP
     `actions_run_trigger`, owner `ivanmagpelmeni`) so the craft-update commit gets a run, and
     meanwhile read `runs/99/tour.log`, `replies.jsonl` and the screenshots.
  2. Fix the press-time visibility failure of the bottom-bar buttons (one cause, most failures),
     then the missing long-press-menu and share ids. Push, re-run, repeat until 0 tour ERR and
     the shots look right.
  3. Merge into main **at the start of a session** (`git checkout main; git merge --ff-only
     origin/main; git merge --no-ff origin/upstream-0.19.0`), push, watch the main run to the
     end.
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
