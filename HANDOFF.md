# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, about 20:25 Berlin, end of nightly session 2 (unattended, started 19:54).
- **Upstream:** `git ls-remote https://github.com/storytold/lightcraft 'refs/tags/*'`: newest tag
  still `v0.9.1` (our fork point). Nothing to merge.
- **Checked:** run 10 screenshots: bottom toolbar clears the home indicator (`safe area (points):
  top 0 left 0 bottom 25 right 0`), mask panel shows exactly 5 tiles. Good.
- **Done this session:**
  - `b4cd395` P6: on-device log. The host now uses upstream's desktop logger
    (`apps/lightcraft/src/logging.rs` via `#[path]`): stderr + `Documents/Logs/lightcraft.log`,
    rotated per launch, panics recorded. Tour checks the file.
  - `63582bd` P7: AI items (Object, Describe…, Sky, Subject) hidden in the mask
    Add/Subtract/Intersect menus on iOS (upstream patch, logged in `UPSTREAM.md`).
  - Both green in **run 12, released as RawPad 1.12** (run 11 was cancelled by the second push).
  - `6bbcf62` P5: **Export › Add to Apple Photos** on iPad: `native::photos_runner` plugs into
    the engine's Photos import (`PHPhotoLibrary performChangesAndWait` on its worker thread);
    upstream gates opened for iOS (export.rs, dialogs.rs; in `UPSTREAM.md`); Info.plist
    `NSPhotoLibraryAddUsageDescription`; CI pre-grants `photos-add`; tour exports once with
    `addToPhotos`. Tested on branch `p5-photos` with `workflow_dispatch`: **run 13 green**, and
    `export.photosImports` returned `"imported": 1` with a Photos asset id.
  - Merged `p5-photos` into main with the doc updates at handoff → **run 14**: at 20:31 Berlin
    fmt, unit tests, sim build, clippy and the **simulator tour had passed**; device build and
    release 1.13 still running.
- **Very next step:** check run 14 (green? release 1.13? `runs/14/replies.jsonl` shows
  `"imported": 1`?). If red, it's the merge only (code identical to green run 13): read the
  logs; revert the merge commit if you can't fix it. Then `PROGRESS.md` → *Next 3 tasks* (share sheet).
- **Open problems:**
  - 24 MP exports fall back to the CPU in the simulator: "GPU render: 6000×4000 needs 275 MiB
    buffers, over the device's 256 MiB storage-buffer limit" (run 13 replies). Phase 6.
  - Long-press and Photos permission are untested on the real iPad (Needs Ivan).
  - Top inset is 0 (status bar hidden), so iPadOS's multitasking "•••" sits over the search box
    in the top bar. Cosmetic; consider a small top inset.
  - Delete the merged `p5-photos` branch when convenient (harmless if left).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show origin/ci-artifacts:LATEST`;
    files under `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, screenshots).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch test: push a branch, then the GitHub tool `actions_run_trigger` (`run_workflow`,
    `ipad.yml`, ref = branch, inputs `{"release": false}`). Branch runs don't cancel main runs.
  - A full run takes about 20 minutes; the tour result is known after about 10.
