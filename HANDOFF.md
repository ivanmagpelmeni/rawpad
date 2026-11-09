# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-09, about 01:35 Berlin, by the scheduled session (00:54–01:40 Berlin).
- **Upstream:** **v0.19.0** (2026-11-06) is still the newest (checked with `git ls-remote --tags`).
- **Main:** 28048beb = merge of `p7-effect-select` (long-press on a Manage Effect Presets row,
  DECISIONS 91). It started **main run 109**
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42538830192), which builds the next
  release. **Don't push code to main until it ends.**
- **Done this session:**
  - Main run 107 (`p7-reset-hints`): success, `runs/107/tour.log` 0 ERR/FAIL, **release v1.107**.
  - Branch run 106 (`p7-effect-select`): success, 0 ERR, the new unit test ran (13 iPad tests
    passed). Merged into main.
  - PROGRESS › Next 3 tasks › 2 (small touch targets) is already done: DECISIONS 53 and 57
    (22 pt grab radius for mask pins/handles, a separate 7 pt rotate knob 44 pt out);
    `runs/105/03-masking.jpg` shows the knob. The audit's "still broken" list is now all done.
  - **New branch `p1-gpu-check`** (Phase 1's leftover: GPU vs CPU pixel comparison, DECISIONS 92):
    the tour exports photo 1 as a 1200 px PNG with Metal, again with `app.gpu {"enabled": false}`,
    and the new `compareImages` step in `ios/sim_test.py` prints the mean / 99.9 % / max
    difference. **Soft check** (warning only). Main was merged into the branch afterwards
    (4b0b4cf6, docs conflict in DECISIONS.md resolved).
  - **Branch run 108** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42537859931) on
    6776480f: build and unit tests green; the simulator tour was still running at handoff.
- **Very next step:**
  1. Check main run 109 and the new release (`…/releases?per_page=2`); `runs/109/tour.log` no ERR.
     If red: fix on main, or `git revert -m 1 28048beb`.
  2. Read run 108's `runs/108/tour.log`: `grep compareImages` gives the numbers; in
     `runs/108/replies.jsonl` the export with `gpu-check-gpu.png` must have **no** `gpuFallback`
     (else the GPU never ran and the check compares CPU with CPU: then try a smaller `longEdge`
     or look at `app.gpu`'s reply just before it). Also check that both exports said ok.
  3. If the numbers are fine and the GPU really ran: set limits a little above what was seen,
     make the step hard (drop `"soft": true`), push, run the branch again, then merge into main
     (after run 109). If the numbers are bad: that's a real Metal rendering difference on the
     iPad — find which stage (export a photo with one slider at a time) before anything else.
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
  - Merged branches to delete on GitHub some time: `p7-reset-hints`, `upstream-0.19.0`, `p7-label-pill`, `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`,
    `p6-hard-checks`, `p6-preview-cap`, `p6-gpu-pool`, `p7-stroke-start`, `p7-effect-select`.
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
