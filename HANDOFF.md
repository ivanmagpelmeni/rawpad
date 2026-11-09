# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-09, about 03:30 Berlin, by the scheduled session (02:54–03:40 Berlin,
  the last run of the night).
- **Upstream:** **v0.19.0** (2026-11-06) is still the newest.
- **Main:** **`p1-gpu-check` merged** (536a3e7e: the GPU-against-CPU export comparison is a
  hard tour check, DECISIONS 93). **Main run 112**
  (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42548144345): **success**, **release
  v1.112** out (SideStore source a77ee959). Report: `runs/112/` on `ci-artifacts` (not published yet at handoff).
- **Done this session:**
  - Branch run 110 (`p1-gpu-check`) read: 0 ERR, `compareImages … mean 0.000, 99.9% 0, max 1`.
  - Branch run 111 (`p7-drop-sysmem`) **failed only in the tour**: the share sheet froze the
    app's frames, so the crash-safety steps after it were refused ("the app drew no frame for
    5 s"), same as runs 48/49. Not caused by the branch. Re-ran it: **branch run 113**
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42548152481): tour **passed**.
  - **New branch `p1-share-last`** (0c835c15, DECISIONS 95): the tour's background export,
    Share button, share sheet and soft Add to Photos step move to the very end, so that freeze
    can't fail later hard steps. **Branch run 114**
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/42548286494): tour **passed**.
- **Very next step:**
  1. Read `runs/112/tour.log` (0 ERR expected; run 112 was green) and look at `08-split.jpg`.
  2. Check runs 113 and 114 finished (device build) green; `runs/113/` and `runs/114/` tour
     logs: no ERR. In `runs/114/`, `05-share-offer.jpg` and `06-share-sheet.jpg` should still
     show the Share button and the sheet (now taken in the square grid after the relaunch).
  3. Merge `p7-drop-sysmem` into main (`git merge --no-ff origin/p7-drop-sysmem`; DECISIONS.md
     conflict: keep 93, then 94). Push, wait for the main run (~40 min). Then merge
     `p1-share-last` (DECISIONS: keep 94, then 95). Never push code while a main run is going.
  4. Then PROGRESS › Next 3 tasks (tiling: measure first; craft-fonts).
- **Open problems:**
  - The share sheet sometimes stops the app drawing frames in the simulator (runs 48, 49,
    111). Worked around in the tour (`p1-share-last`); on the iPad it shouldn't matter (RawPad
    redraws when the sheet closes), but if Ivan says the screen freezes after sharing, look here.
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
    `p6-gpu-pool`, `p7-stroke-start`, `p7-effect-select`, `p1-gpu-check`.
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
