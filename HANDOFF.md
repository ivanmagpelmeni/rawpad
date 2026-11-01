# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-01, about 22:30 Berlin, by the scheduled session (21:55–22:35 Berlin,
  the fourth run of the night).
- **Upstream:** not re-checked this session (checked at 19:54 Berlin: newest tag `v0.9.1`).
- **Main:** run 81 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41347106631) green,
  **released 1.81** (the `p6-relaunch` merge). Its tour: memory warning 524 → 246 MB resident,
  relaunch checks passed. Main is f581546a (CI's "SideStore source: RawPad 1.81") plus this
  session's docs commit.
- **Done this session:**
  - Read run 80's report: all three soft relaunch checks passed (suspend line logged, 22 records
    replayed, `selection` = [2]); shot `14-relaunched` shows photo 2 open in the loupe, 4 stars.
  - Branch **`p6-hard-checks`** (DECISIONS 73; commits 3e998be8, 2586b395): those checks are
    hard, a new hard `loupe.photo` = 2; `maxRssMB` hard at 1500 on every memory step; new
    `maxEngineMB` in `sim_test.py` (ui.inspect › memory › engineBytes), hard at 1200; a final
    `memory` step `grid-after-relaunch` in the square grid with shot `15-grid-after-relaunch`.
  - Branch run 83 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41349422537):
    build, unit tests, clippy green; the tour was still running at 22:33 Berlin
    (result unknown). **Not merged.**
- **Very next step:**
  1. If run 83 is green and not merged yet: look at `runs/83/15-grid-after-relaunch.jpg` and
     `memory.jsonl` (the `grid-after-relaunch` line), then merge `p6-hard-checks` into main
     (`git merge --no-ff origin/p6-hard-checks`; fetch first: CI pushes the SideStore commit).
     If a hard check failed: read `tour.log`, fix it or make only that check soft again with a
     DECISIONS entry.
  2. Then PROGRESS.md › Next 3 tasks › 2 (preview downscaling: `previewSources` is about 52 MB
     per photo, 4 kept).
- **Open problems:**
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Add to Apple Photos in the simulator stays flaky (soft check).
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`.
  - The artifact zip download URL (blob.core.windows.net) is blocked from the sandbox: wait for
    the `ci-artifacts` report.
  - `ui.inspect`'s `window` is [820, 1110] pt in portrait on the CI simulator; the `simctl`
    screenshot stays in the portrait framebuffer, so landscape shots look turned sideways.
  - `ci-artifacts` `LATEST` can name a cancelled run's number (it said 82 while run 82 was
    cancelled): always list `runs/` and pick the folder yourself.
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
    `git reset --hard` is refused in the sandbox: use `git checkout -B main origin/main`.
    `git fetch origin main` overwrites FETCH_HEAD: fetch `ci-artifacts` again before `git show`.
  - A memory step's numbers: `runs/<n>/memory.jsonl` (one JSON line per `memory` tour step).
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once. A push to a branch doesn't start a run, so push every commit
    before triggering. CI only runs the `tests_curve` module of the UI crate's unit tests, so
    app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. Panels: `panel.edit`, `panel.presets` (toggle!), `panel.versions`, `panel.activity`;
    to open a panel for sure use `ui.set {"right": "edit"}` (also `openSections`).
  - `crate::i18n::tr_format!` accepts only messages already in the translations; build new text
    from existing messages or plain `tr`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time). Shell commands time out
    after 2 minutes: wait for CI with `sleep 115` per call.
