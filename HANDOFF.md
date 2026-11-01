# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-01, about 19:30 Berlin, by the scheduled session (18:54–19:40 Berlin, the
  first run of the night). The previous session was 2026-10-26: nothing ran in between.
- **Upstream:** checked (`git ls-remote --tags`): newest tag still `v0.9.1`, nothing to merge.
- **Main:** run 72 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40259845067) green,
  **released 1.72** (low-field keyboard fix). This session **merged `p4-rotate`** into main (tour
  and docs only); main's run for it releases about 1.74: check it.
- **Done this session (DECISIONS 68, 69):**
  - Run 72's `10-select-several-synced` shows the "Sync Settings: 2 photos" toast (fading): checked.
  - Keyboard height: the app's 257 pt matches the screenshot (keyboard top at 898 of 1180 pt,
    minus the 25 pt safe area). Off the queue.
  - **Rotation in the tour:** `ios/sim_test.py` has a new `hostMenu` step (System Events clicks a
    Simulator menu item, like `hostKeys`) and `waitUntil … "wider": true/false` (reads
    `ui.inspect` › `window`). The tour turns *Device › Rotate Left*, waits for a wide window, shoots
    `12-landscape-loupe` and `12-landscape-grid`, turns *Rotate Right*, waits for portrait, shoots
    `12-portrait-again`.
  - **Branch run 73** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41334044014): all
    green; both rotations worked (`ok hostMenu`, `ok waitUntil window`); shots checked: the whole
    UI laid out wide in landscape, toolbar clear of the home indicator, portrait right after. No new
    "safe area" log line after turning (insets unchanged), so that soft check was dropped and the
    two waits became hard checks (this last tweak not yet run: main's run is its first).
- **Very next step:**
  1. Check main's run for the `p4-rotate` merge is green (it runs the hard rotation checks for the
     first time); fix or revert the tour step if red.
  2. Then PROGRESS.md › Next 3 tasks: a 24 MP raw memory test in the simulator, then memory
     warnings.
- **Open problems:**
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`.
  - The artifact zip download URL (blob.core.windows.net) is blocked from the sandbox: wait for
    the `ci-artifacts` report.
  - `ui.inspect`'s `window` is [820, 1110] pt in portrait on the CI simulator (1180 minus the top
    gap and safe area); the `simctl` screenshot stays in the portrait framebuffer, so landscape
    shots look turned sideways.
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
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time).
