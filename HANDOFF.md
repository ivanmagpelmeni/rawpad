# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-19, about 00:36 Berlin, end of nightly session 6 (unattended, started 23:55).
- **Upstream:** not checked (not the first run of the night).
- **Done this session** (branch `p4-batch` = `p4-pinch` + `p4-top-inset` + `p5-share-label` + a CI
  fix, merged into main at handoff):
  - **Tour pinch-zooms and pans the loupe** (`ios/sim_tour.jsonl`, shots `04-pinch-zoom`,
    `04-panned`): `ui.zoom` 3× → 45 %, two-finger scroll and one-finger drag move `ui.pan`. Seen in
    run 22's screenshots (navigator box, pan moved). `sim_test.py` `waitUntil` has `differs`.
  - **Top inset 20 pt** for the multitasking "•••" (`ios.rs::MIN_TOP_INSET`). Run 23 tour green;
    **screenshot not looked at yet** (its report didn't land on ci-artifacts): check
    `runs/<main run>/01-library-grid.jpg`: the dots must sit above the search box.
  - **Share Exported Photo… translated** (`share_offer::label`, 14 languages, unit-tested).
  - **CI Photos warm-up** (`simctl addmedia` after boot) and a 240 s export wait: runs 22/24 failed
    because Add to Photos took 92 s+ on a cold simulator (DECISIONS.md 37).
  - Run 25 (`p4-batch`, https://github.com/ivanmagpelmeni/rawpad/actions/runs/39171674785): fmt,
    unit tests, clippy and the **tour green**; device build was still running at merge.
- **Very next step:** check the main run started by the merge push (likely run 26): green and
  released as 1.26? Look at `01-library-grid.jpg` (top gap) and `05-share-offer.jpg`, and the
  "Photos warm-up took N s" line in the job log. If red, fix or revert the merge first.
- **Then:** phase 4: Apple Pencil for brush masks (Pencil = touch with `force`/stylus; check how
  egui-winit reports it), then hardware-keyboard shortcuts (⌘ keys through winit on iOS).
- **Open problems:**
  - Real-finger pinch/pan, long-press, Photos permission and the share sheet are untested on the
    real iPad (Needs Ivan).
  - Deleting the merged branches (`p5-photos`, `p5-share`, `p5-share-export`, now also `p4-pinch`,
    `p4-top-inset`, `p5-share-label`, `p4-batch`) failed from this sandbox (`git push --delete`
    disconnected). Try the GitHub tools or leave them.
  - The one-finger drag pan only moved x (image fits vertically at 45 %: expected).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show origin/ci-artifacts:LATEST`;
    files under `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, screenshots). On a red
    tour the report is published right after (no device build). Parallel runs can lose a report
    (run 23's never landed).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `actions_run_trigger`, `ipad.yml`, ref = branch, `{"release": false}`. They don't
    cancel main runs. A full run takes 20–30 minutes; the tour result is known after about 12–15.
