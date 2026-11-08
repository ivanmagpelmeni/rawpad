# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-08, about 19:35 Berlin, by the scheduled session (18:56–19:40 Berlin,
  the first run of the night).
- **Upstream:** **v0.19.0** (2026-11-06) is the newest tag; main is still on v0.9.1. Earlier
  sessions missed v0.10–v0.19 because plain `sort` puts `v0.9.1` last (DECISIONS 85: use `sort -V`).
- **Main:** `p7-label-pill` merged at the start of this session (a20dd94a, branch run 95 green).
  Main run 96 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42507547011) builds and
  releases it (about 1.97). At 19:35 Berlin its tests, tour and device build had passed and it was saving the cache before the release step; check `…/releases?per_page=2` for 1.97.
- **Done this session:**
  - Branch **`upstream-0.19.0`**: merge commit 876cbdc0 (parents: main and upstream v0.19.0;
    made with `git merge-recursive v0.9.1 -- HEAD v0.19.0` because the fork has no upstream
    history, DECISIONS 83). Conflicting upstream UI files: upstream's version taken; old patches
    saved in `ios/upstream-patches-v0.9.1.diff`.
  - Then five commits re-applying **every iPad patch** in upstream's new file layout (UPSTREAM.md
    › "Paths after 0.19.0"; each patch has a `RawPad` comment: `grep -rn RawPad crates/`).
  - egui is 0.37 on storytold's fork: `egui::Id::new` is **`egui::Id::unique`** there (changed in
    the host too); host `egui-winit`/`egui-wgpu` bumped to 0.37 (patched to the fork by the
    workspace). Cargo.lock came from upstream and has no `rawpad-ios` entry: CI's cargo (not
    `--locked`) adds it.
  - iPad UI tests moved to `crates/ui-egui/src/tests/ipad.rs` (DECISIONS 84); `ipad.yml` runs
    `--lib tests::ipad`.
  - Branch run 98 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42508300021) on
    70e3eddb: at 19:35 Berlin **fmt, the host unit tests and the UI unit tests
    (`tests::ipad`, so upstream 0.19.0 + all re-applied patches compile and the iPad tests pass)
    were green**; "Build for the simulator" (the first compile of the host for iOS against egui
    0.37) was still running. Check that `ui-tests.log` really ran the ten iPad tests (not "0 tests").
- **Very next step:**
  1. Look at branch run 98 (or the latest run on `upstream-0.19.0`). Compile errors are in
     `runs/<n>/unit-tests.log` (host crate), `ui-tests.log` (UI crate) or `build-sim.log`; if the
     report wasn't published (early failure), read the job log with the GitHub MCP
     `get_job_logs` (owner `ivanmagpelmeni`, `failed_only: true`, `return_content: true`).
     Likely spots: the host's egui-winit / egui-wgpu / winit calls (`apps/rawpad-ios/src/ios.rs`)
     after the 0.37 bump; `register(ui, …)` now takes the `Ui`, not the context; the copied tests in
     `tests/ipad.rs` (harness fields like `h.view.ctx`, `h.app.widgets` may have moved).
  2. Fix, push, trigger `ipad.yml` on `upstream-0.19.0` with `{"release": false}`; repeat until
     green, then look at the tour screenshots (widget ids may have changed upstream; the tour is
     `ios/sim_tour.jsonl`).
  3. When green with 0 tour ERR and the shots look right: merge into main **at the start of a
     session** (`git checkout main; git merge --ff-only origin/main; git merge --no-ff
     origin/upstream-0.19.0`), push, watch the main run to the end. Check craft-update (the
     upstream self-updater, new in ui-egui) doesn't offer desktop updates on the iPad.
  4. Then PROGRESS.md › Next 3 tasks.
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
