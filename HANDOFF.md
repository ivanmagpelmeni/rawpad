# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-01, about 21:35 Berlin, by the scheduled session (20:54–21:40 Berlin,
  the third run of the night).
- **Upstream:** not re-checked this session (checked at 19:54 Berlin: newest tag `v0.9.1`).
- **Main:** run 79 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41342404278) green,
  **released 1.79** (the `p4-raw24` merge). Run 79's `memory.jsonl` (resident MB / engine MB):
  before-raw 147 / 582, raw-open 193 / 584, edited 93 / 589, zoomed 376 / 610, after 522 / 589.
- **Done this session:**
  - Branch run 78 (`p6-memtrim`) green: unit tests `memtrim::*` pass, the Simulator's *Debug ›
    Simulate Memory Warning* reaches the app, log "caches held 587.1 MB, now 324.7 MB". The check
    is hard now (commit 47338afc).
  - **Crash safety** (DECISIONS 72, commit 21b78011): `sim_test.py` has `background` (launch
    another app, e.g. Settings) and `relaunch` (`simctl terminate`, launch again with the same env,
    reconnect; the log continues in `app-stderr-2.log`, stdout in `app-stdout-2.log`). The host
    logs "suspended: library and view saved (N changes unsaved)". The tour's last block rates
    photo 2 → 4, opens it, backgrounds, kills, relaunches and checks: library line in the new
    log (hard), ≥ 24 photos (hard), photo 2 rated 4 (hard), suspend line / non-zero replay /
    `selection` = [2] (soft). Shot `14-relaunched`. Memory warning moved right after the raw
    test with a new `after-memwarn` memory step.
  - **Branch run 80** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41344375571, branch
    `p6-relaunch` incl. the memtrim merge): build, unit tests, clippy and the tour step green
    (so the hard relaunch checks passed); the device build was still running. **Merged into
    main** (commit b3e4b830); main's run releases about 1.81.
- **Very next step:**
  1. Check main's run for b3e4b830 is green (releases about 1.81). If red: the only code change
     is one `log::info!` in `ios.rs` `suspended` and the tour/sim_test changes; revert the merge
     if it can't be fixed quickly.
  2. Read run 80's (or main's) report: `tour.log` from `relaunch:` on (did terminate leave the app
     gone? rc of the launch?), the three soft checks, `app-stderr-2.log`, shot `14-relaunched`
     (must show photo 2 open in the loupe, not the launch screen). Make passed soft checks hard.
     If the suspend line is missing: winit's `suspended` may not fire on backgrounding in the
     simulator; check `app-stderr.log` around the `background` step.
  3. `memory.jsonl`: compare `after-raw` vs `after-memwarn` (the trim's effect on resident size).
     Then tighten `maxRssMB` (soft 2500 now; make it hard at about 1500).
- **Open problems:**
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Add to Apple Photos in the simulator stays flaky (run 78: soft ERR/WARN as before).
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`.
  - The artifact zip download URL (blob.core.windows.net) is blocked from the sandbox: wait for
    the `ci-artifacts` report.
  - `ui.inspect`'s `window` is [820, 1110] pt in portrait on the CI simulator; the `simctl`
    screenshot stays in the portrait framebuffer, so landscape shots look turned sideways.
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
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time). Shell commands time out
    after 2 minutes: wait for CI with `sleep 115` per call.
