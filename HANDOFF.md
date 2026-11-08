# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-08, about 23:35 Berlin, by the scheduled session (22:55–23:40 Berlin).
- **Upstream:** **v0.19.0** (2026-11-06) is the newest tag and is now **merged on main**
  (a0de4dbb, from branch `upstream-0.19.0`; UPSTREAM.md updated). Branch `upstream-0.19.0` can
  be deleted some time.
- **Main:** 80906a2d (merge + UPSTREAM.md) started **main run 104**
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42527682613), which also builds release
  1.104. At handoff (23:34 Berlin) it had passed fmt, both unit-test steps and the simulator build (slow: about 30 minutes, cache rebuilt after the merge) and was in clippy; tour, device build and release still to come. **Don't push code to main until it ends.**
- **Done this session:**
  - Checked branch run 103: success, `tour.log` 0 ERR/FAIL, `08-rating-menu`, `08-split`,
    `13-raw-edited` screenshots correct. Merged into main (only UPSTREAM.md conflicted: took the
    branch's version).
  - Branch **`p7-reset-hints`** (027a9c17 + docs): on iOS / in unit tests a long-press on a curve
    channel button opens the curve reset menu for that channel; a long-press on a colour wheel
    opens "Reset <wheel>" (`wheelMenu:reset:<key>`). DECISIONS 90, UPSTREAM rows, tests
    `a_curve_channels_long_press_menu_resets_it`, `a_colour_wheels_long_press_menu_resets_it`.
  - **Branch run 105** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42527881509):
    fmt, both unit-test steps (with the two new tests), simulator build and clippy green; the tour was running at handoff. Report lands in `runs/105/`.
- **Very next step:**
  1. Check main run 104's conclusion and `runs/104/tour.log` (grep ERR) on `ci-artifacts`, and
     that release 1.104 exists (`…/releases?per_page=2`). If red: fix on main first (or revert
     the merge with `git revert -m 1 a0de4dbb` if it can't be fixed in the session).
  2. Check `runs/105/tour.log` (no ERR) and the `11-*.jpg` screenshots. If clean and no main
     run is going: `git checkout main; git merge --ff-only origin/main; git merge --no-ff
     origin/p7-reset-hints`, push, watch the main run.
  3. Then PROGRESS › Next 3 tasks › 2 (the other hover-only hints: crop straighten Auto, History).
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
  - The upstream merge (now on main) took ten releases at once; upstream tests that measure the bottom-bar pill
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
