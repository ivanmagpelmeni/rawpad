# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-25, about 20:35 Berlin, by the scheduled session (19:54–20:35).
- **Upstream:** checked (`git ls-remote --tags`); newest tag still `v0.9.1`.
- **Main:** run 47 green, **released 1.47** (touch grabs, curve Delete Point, Crop Done).
  Run 46's `09-crop-done-button` / `09-crop-applied` screenshots checked: Done next to Reset,
  back in Edit with no crop frame afterwards.
- **Done this session (on branches, not merged yet):**
  - `p4-touch-2` (2 commits): radial mask rotate knob 44 pt out from the right-edge handle and
    drawn larger on iPad; targeted adjustment samples at `press_origin()`; rating stars take a
    36 pt tall touch area. Run 48
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40223693146): compile, clippy, unit
    and ui tests green; screenshot `runs/48/03-masking.jpg` shows the bigger knob out to the right.
    The tour **failed at its very last step** (Add to Apple Photos export after the share sheet:
    "the app drew no frame for 5 s"), which none of these changes touch → judged a simulator flake;
    **failed job re-run started at 20:31 Berlin** (same run id, attempt 2).
  - `p4-select-several` = `p4-touch-2` + *Select Several* in the photo long-press menu (taps then
    toggle photos in the selection, for Sync; DECISIONS 58 on that branch) + a CI tour step
    (`10-select-several-menu`, `10-select-several`, expects 3 selected) + colour-wheel eye always
    shown on iPad (28 pt touch area; commit `838583f`, pushed after run 49 started, so **not yet
    built**) + DECISIONS 57–58 and UPSTREAM.md rows. Run 49
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40223846501): compile, clippy, tests
    green; tour was still running at handoff.
- **Very next step:**
  1. Run 49 result: tour log `runs/49/tour.log` (on `ci-artifacts`), look for the `selected = 3`
     waitUntil and the `10-select-several*.jpg` shots (menu with *Select Several* at top; three
     highlighted cells). If it failed only at the Add-to-Photos step again like run 48, that step
     is flaky after the share sheet now: make it `soft`/move it before the share sheet.
  2. Run 48 attempt 2 result (flake confirmation).
  3. Dispatch `ipad.yml` on `p4-select-several` again (`{"release": false}`) to build the
     colour-wheel commit; when green, merge `p4-select-several` into main (it contains
     `p4-touch-2`) with a merge commit, push; main's run releases about 1.5x.
  4. Then PROGRESS.md › Next 3 tasks.
- **Open problems:**
  - Run 48's last-step failure (above).
  - *Select Several* has no on-screen "on" indicator other than highlighted cells.
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Keyboard-shrink path only unit-tested. Real-device checks need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all` (and `p4-touch-2` once merged).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `ui-tests.log`, screenshots).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`). A full run takes 25–35 minutes.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 since today (winter time).
