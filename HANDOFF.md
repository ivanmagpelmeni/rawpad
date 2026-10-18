# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, about 22:42 Berlin, end of nightly session 4 (unattended, started 21:55).
- **Upstream:** not checked (not the first run of the night; 19:54 found nothing newer than `v0.9.1`).
- **Checked:** run 16 (main, share-sheet merge) green, released **1.16**; `runs/16/05-share-sheet.jpg`
  shows the sheet for the log.
- **Main is unchanged and green** (only these docs were pushed to main this session).
- **In progress on branch `p5-share-export`** (not merged):
  - `apps/rawpad-ios/src/share_offer.rs`: after an export finishes, a **Share Exported Photo…** button
    above the toast for 20 s; tap → one share sheet for all written files (`native::share_files`).
    Host draws it in `ios.rs::frame` and registers `button:rawpadShareExport` in `app.widgets`.
    Unit tests pass (run 18/19). DECISIONS.md 31.
  - `ios/sim_test.py`: new `waitUntil` step (DECISIONS.md 32). Tour now exports photo 2 with
    `"background": true` (the Export dialog's path) and waits for `export.last`.
  - Runs: 17 (test panicked on texture deltas, fixed), 18 (tour: no button, because control exports
    without `background` never set `last_export_result`), **19 red**:
    https://github.com/Ivanmagpelmeni/rawpad/actions/runs/39163067682
- **Found in run 19 (important, likely affects released 1.14/1.16):** a *background* export with
  Add to Apple Photos hangs: the activity row sat at "Adding to Apple Photos…" for 90 s and
  `export.photosImports` stayed `running: true`. The synchronous control export (runs 13–18) worked.
  The Export dialog uses the background path, so on the iPad *Export with Add to Apple Photos ticked*
  probably never finishes. Suspect: `native::photos_runner` uses `performChangesAndWait`, and in the
  background path something waits on the main thread (deadlock), or the runner is called from a
  thread it doesn't expect. Look at `crates/ui-egui/src/export_task.rs` (`poll_photos`, how `after`
  runs in `start`) vs. the synchronous path in `control.rs` (~line 530), and at `photos_runner` in
  `native.rs`.
- **Very next step:** fix the hang (switch the runner to the async `performChanges:completionHandler:`
  or make sure it never runs/waits on the main thread), keep the tour as on the branch, re-run the
  branch (`actions_run_trigger`, `ipad.yml`, ref `p5-share-export`, `{"release": false}`). When
  green, look at `runs/<n>/05-share-offer.jpg` (button above the toast) and `06-share-sheet.jpg`,
  then merge into main. If the hang can't be fixed quickly: make the tour's background export
  *without* `addToPhotos` (keep the synchronous Photos export step from before) so the share button
  can be merged, and keep the hang as the top open problem.
- **Open problems:**
  - The background Photos hang above.
  - Long-press, Photos permission and the share sheet are untested on the real iPad (Needs Ivan).
  - Top inset is 0 (status bar hidden), so the multitasking "•••" sits over the search box.
  - The share button's label is English only.
  - Delete merged branches `p5-photos`, `p5-share` when convenient.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show origin/ci-artifacts:LATEST`;
    files under `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, screenshots). On a red
    tour the report is published right after (no device build).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs don't cancel main runs. A full run takes 20–30 minutes; the tour result is known
    after about 15.
