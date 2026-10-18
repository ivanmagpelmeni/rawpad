# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, about 23:20 Berlin, end of nightly session 5 (unattended, started 22:54).
- **Upstream:** not checked (not the first run of the night; 19:54 found nothing newer than `v0.9.1`).
- **Done this session:**
  - **Photos hang fixed** (`apps/rawpad-ios/src/native.rs::save_to_photos`): the async
    `performChanges:completionHandler:` plus a channel wait with the runner's limit, instead of
    `performChangesAndWait`, which never returned on the export worker thread (DECISIONS.md 33).
    Branch run 20 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39165578598) green: the
    background export with Add to Apple Photos finished, `imported: 1`, log lines
    `photos: saving …` / `photos: saved …` 0.8 s apart.
  - Looked at `runs/20/05-share-offer.jpg` (the **Share Exported Photo…** button with ✕ at the
    bottom of the loupe) and `06-share-sheet.jpg` (the sheet for `LC01214.jpg`). Both right.
  - **Merged `p5-share-export` into main** (merge commit pushed with these docs). That push starts
    the main run, which should release it (version = 1.<run number>, likely 1.21).
- **Very next step:** check that main run (`https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`)
  is green and released, and glance at its `05-share-offer.jpg`. If red, fix or revert the merge
  (`git revert -m 1 <merge>`). Then phase 4: add a pinch-zoom step to the tour (`ui.zoom` or the
  control protocol's pointer/zoom request; see `docs/control-protocol.md`), and check touch → pan in
  the loupe.
- **Open problems:**
  - Long-press, Photos permission, the share sheet and the Share button are untested on the real
    iPad (Needs Ivan).
  - Top inset is 0 (status bar hidden), so the multitasking "•••" sits over the search box.
  - The share button's label is English only.
  - Delete merged branches `p5-photos`, `p5-share`, `p5-share-export` when convenient.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show origin/ci-artifacts:LATEST`;
    files under `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, screenshots). On a red
    tour the report is published right after (no device build).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `actions_run_trigger`, `ipad.yml`, ref = branch, `{"release": false}`. They don't
    cancel main runs. A full run takes 20–30 minutes; the tour result is known after about 12.
