# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-01, about 23:40 Berlin, by the scheduled session (22:55–23:40 Berlin,
  the fifth run of the night).
- **Upstream:** not re-checked this session (checked at 19:54 Berlin: newest tag `v0.9.1`).
- **Main:** run 84 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41354485607), the
  `p6-hard-checks` merge: tour passed, green; it releases about 1.84. Then `p6-preview-cap` was
  merged on top (its main run releases about 1.86; not watched by this session).
- **Done this session:**
  - Run 83's report: hard relaunch checks passed; `15-grid-after-relaunch` shows the grid with
    photo 2 selected and 4 stars; `grid-after-relaunch`: 264 MB resident, 174 MB engine.
    `p6-hard-checks` merged (e230e59e).
  - Branch **`p6-preview-cap`** (DECISIONS 74, f4db3660): `memtrim::set_ipad_limits` sets
    `session.media.preview_capacity = 3` in `ios.rs` › `start` (no upstream patch); unit test
    `the_ipad_keeps_at_most_three_loupe_sized_decodes`; `sim_test.py` memory steps take a hard
    `maxPreviews` (3 on every memory step after `before-raw`).
  - Branch run 85 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41354581646) green:
    `previewSources` count 3 at every step (157 MB instead of 210 MB); shot `08-side-by-side`
    looks right. Merged into main after run 84.
  - DECISIONS 75: no extra thumbnail cap for now.
- **Very next step:**
  1. Check the main run after the `p6-preview-cap` merge (the newest `main` run): green, and its
     `memory.jsonl` shows `previewSources` count ≤ 3. If HANDOFF's merge didn't happen
     (`git log origin/main` has no "Merge p6-preview-cap"), merge it now
     (`git merge --no-ff origin/p6-preview-cap`, fetch first).
  2. Then PROGRESS.md › Next 3 tasks › 2 (GPU pool after zooming a 24 MP raw: 117 → 513 MB).
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
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`,
    `p6-hard-checks`, `p6-preview-cap`.
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
