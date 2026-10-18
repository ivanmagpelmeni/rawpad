# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, about 21:30 Berlin, end of nightly session 3 (unattended, started 20:55).
- **Upstream:** not checked (not the first run of the night); last check at 19:54 found nothing
  newer than `v0.9.1`.
- **Checked:** run 14 (main, Photos merge) green, released **1.14**; its tour exported to Photos
  (the import was still `running` when queried).
- **Done this session:**
  - `P5: share sheet on iPad`: `native::share` presents `UIActivityViewController` for a file
    (popover anchored at the last pointer position, else mid-window). It is the host's
    `Services::reveal`, so every *Show in Finder* (Photo menu / grid context menu for an original,
    a folder's context menu, Help › Open Log Folder) shares instead. `reveal_label()` returns
    "Share…" on iOS (upstream patch, logged in `UPSTREAM.md`). `Services::log_file` is now set,
    so Help › Open Log Folder is enabled and shares `lightcraft.log`. DECISIONS.md 29.
  - Tour: polls `export.photosImports` once more, then invokes `app.openLogFolder` (expectOk) and
    takes `05-share-sheet`.
  - Tested on branch `p5-share` with workflow_dispatch: **run 15 green**
    (https://github.com/Ivanmagpelmeni/rawpad/actions/runs/39156540949). `runs/15/05-share-sheet.jpg`
    shows the sheet ("lightcraft · Text Document", Copy, Add Tags, **Save to Files**), app still
    running at the end.
  - Merged `p5-share` into main with these docs → **run 16** (should release 1.16).
  - DECISIONS.md 30: the 24 MP GPU fallback is treated as simulator-only (engine already asks
    for `adapter.limits()`; the simulator reports 256 MiB).
- **Very next step:** check run 16 (green? release? `runs/16/05-share-sheet.jpg`). If red, it is
  the merge only (code identical to green run 15): read the logs; revert the merge if you can't
  fix it. Then `PROGRESS.md` → *Next 3 tasks* #2 (Share right after an export).
- **Open problems:**
  - The Photos import reads `"running": true` 10 s and 15 s after the export in the simulator
    (runs 14, 15); the toast's bar is nearly full. Make the tour poll it (Next 3 tasks #3).
  - Long-press, Photos permission and the share sheet are untested on the real iPad (Needs Ivan).
  - Top inset is 0 (status bar hidden), so the multitasking "•••" sits over the search box.
  - "Open Log Folder" keeps its upstream label though it shares the log file on iPad. Fine for now.
  - Delete the merged `p5-photos` and `p5-share` branches when convenient (harmless if left).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show origin/ci-artifacts:LATEST`;
    files under `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, screenshots). The
    report is published only after the device build (~30 min into a branch run with a cold cache).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch test: push a branch, then the GitHub tool `actions_run_trigger` (`run_workflow`,
    `ipad.yml`, ref = branch, inputs `{"release": false}`). Branch runs don't cancel main runs.
  - A full run takes 20–30 minutes; the tour result is known after about 15.
