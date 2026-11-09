# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-09, about 02:35 Berlin, by the scheduled session (01:54–02:40 Berlin).
- **Upstream:** **v0.19.0** (2026-11-06) is still the newest.
- **Main:** unchanged code since 28048beb (merge of `p7-effect-select`). **Main run 109**
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42538830192): success,
  `runs/109/tour.log` 0 ERR/FAIL, `08-split.jpg` looked at (split before/after fine),
  **release v1.109** out. Only docs pushed to main this session.
- **Done this session:**
  - Run 108's GPU check read: GPU export had **no** `gpuFallback`, its PNG differed from the
    CPU one by a few bytes (so Metal really ran), `compareImages`: mean 0.000, 99.9 % 0, max 1.
  - **`p1-gpu-check`** (4a99796e): the check is hard, limits mean ≤ 0.5 / 99.9 % ≤ 4, plus two
    new generic step keys in `ios/sim_test.py`: `expectEquals {"result.available": true}` on
    `app.gpu` and `expectAbsent ["result.files.0.gpuFallback"]` on the GPU export (DECISIONS 93).
    **Branch run 110** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42542856854):
    the tour step **passed** (so the hard check passed); the device build was still going at
    handoff. Report: `runs/110/`.
  - **New branch `p7-drop-sysmem`** (from main): removes `lightcraft-sysmem` from
    `apps/rawpad-ios/Cargo.toml` (DECISIONS 94; the committed Cargo.lock has no `rawpad-ios`
    entry at all, CI adds it each build, so no lock edit is needed). **Branch run 111**
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42542925912): unit tests, iOS build
    and clippy green; the tour was still running at handoff.
- **Very next step:**
  1. Check runs 110 and 111 finished green; `runs/110/tour.log`: `grep compareImages` (expect
     `ok … max 1`), no ERR; `runs/111/tour.log` no ERR.
  2. Merge `p1-gpu-check` into main (`git merge --no-ff origin/p1-gpu-check`; DECISIONS.md and
     docs conflict: keep main's HANDOFF/PROGRESS, keep entry 93). Push, wait for that main run
     (~40 min, release), then merge `p7-drop-sysmem` (DECISIONS conflict: keep 93 and 94, in
     order). Don't push code to main while a main run is going.
  3. Then PROGRESS › Next 3 tasks › 2 (measure before tiling).
- **Open problems:**
  - Returning free pages (DECISIONS 77) freed 0 MB in the simulator; only a real iPad's log
    ("memory warning: returned X MB") would tell. Resident size after zooming out stays ~550 MB.
  - A full-size 6000×4000 render needs 275 MiB of GPU buffers, over the simulator's 256 MiB
    limit, so it falls back to the CPU (`app.gpu` › `lastFallback`). Slower, not broken. This
    is what tiling (Phase 6) would fix; measure on the iPad first.
  - Whether a 64 MB pool slows panning a zoomed raw is not measured.
  - Split View / Stage Manager resizing can't be scripted in the simulator: Ivan's item 8.
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Add to Apple Photos in the simulator stays flaky (soft check); the import wait in the tour
    warns every run (`imports.imported never became true`, soft).
  - Upstream `bottombar.rs` tests may fail under `cfg(test)` (label button on in tests). CI
    doesn't run them.
  - Merged branches to delete on GitHub some time: `p7-reset-hints`, `upstream-0.19.0`,
    `p7-label-pill`, `audit-2026-10-19`, `p4-curve-touch`, `p4-touch-all`, `p4-touch-2`,
    `p4-select-several`, `p4-select-bar`, `p4-select-sync`, `p4-reset-menu`, `p4-sync-test`,
    `p4-preset-preview`, `p4-snapshot-preview`, `p4-film-select`, `p4-keyboard-check`,
    `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`, `p6-hard-checks`, `p6-preview-cap`,
    `p6-gpu-pool`, `p7-stroke-start`, `p7-effect-select`.
  - The artifact zip URL (blob.core.windows.net) is blocked from the sandbox: use `ci-artifacts`.
  - Landscape `simctl` screenshots look turned sideways (portrait framebuffer).
  - `ci-artifacts` `LATEST` can name a cancelled run: list `runs/` and pick the folder yourself.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
    Job logs (GitHub MCP `get_job_logs`) only exist once the whole job has ended (404 before).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `ui-tests.log`, screenshots).
    Published about 10 minutes after the tour ends. Compile errors of the UI tests are in
    `ui-tests.log`. raw.githubusercontent.com is blocked from the sandbox: use `git fetch`.
  - Pushing code to main while a main run is going cancels it. Markdown-only pushes don't start
    a run. CI pushes `SideStore source: RawPad x.y` to main after each release: fetch first.
    `git reset --hard` is refused in the sandbox: `git checkout -B main origin/main` works.
    `git fetch origin main` overwrites FETCH_HEAD: fetch `ci-artifacts` again before `git show`.
  - Memory step numbers: `runs/<n>/memory.jsonl`.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). 25–35 minutes; two branch runs
    can go at once. A push to a branch doesn't start a run. CI only runs the `tests::ipad`
    module of the UI crate's unit tests (`crates/ui-egui/src/tests/ipad.rs`).
  - Tour step keys (`ios/sim_test.py`): `expectOk`, `capture`, `expectEquals`, `expectAbsent`,
    `expectPhotosAtLeast`, `waitUntil`, `checkLog`, `checkFiles`, `clickVisible`,
    `compareImages`, `memory`, `shot`, `soft`.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. To open a panel for sure use `ui.set {"right": "edit"}`.
  - `crate::i18n::tr_format!` accepts only messages already in the translations.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1. Shell commands time out after 2 minutes:
    `sleep 110` per call at most.
