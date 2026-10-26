# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-26, about 03:38 Berlin, by the scheduled session (02:54–03:40 Berlin, the
  last run of the night).
- **Upstream:** not re-checked (not the first run of the night).
- **Main:** run 70 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40253633287) green,
  **released 1.70** (Select Several pill in the loupe). Then this session **merged
  `p4-keyboard-check`** (merge `c11c712`). Main's run for that merge releases about 1.72: check it.
- **Done this session (DECISIONS 67):**
  - Fixed run 69's keyboard flicker for a low text field. Cause confirmed by reading the code:
    egui reports the edited box (`ime`) only while it is drawn; after the lift the box lay below
    its shrunk panel, so the host dropped the keyboard, the UI grew back, and so on.
    - `apps/rawpad-ios/src/keys.rs` `hold_field` (+ unit test
      `a_focused_field_out_of_view_keeps_the_keyboard`): while the keyboard is up and egui still
      focuses the same box, the last box is kept. `ios.rs` logs "keyboard: focused text field out
      of view, on-screen keyboard kept up" once per stretch.
    - Upstream patch `crates/ui-egui/src/text_field.rs` `KEEP_IN_VIEW` / `keep_in_view` (also the
      Info panel's GPS box in `panels/right.rs`): for 30 frames after the lift the host sets the
      flag and a focused field not fully in view calls `scroll_to_me`. UPSTREAM.md row added.
    - Tour: soft `checkLog` for the "kept up" line; the search field's `checkLog` waits 10 s
      (run 69 saw an empty log after 5 s although the app had logged it).
  - **Branch run 71** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40256706796):
    format, unit tests, clippy, build and the whole tour **green** (the tour-wait commit `550bb7d`
    came after it; JSON only). **Checked its report** (`ci-artifacts` `runs/71/`):
    `04-low-field-keyboard.jpg` shows the keyboard up and the focused Location box ("Dolomites")
    just above it; `app-stderr.log` has "lifting the UI" **once**, then "kept up" once, "UI shrunk
    by 257 pt", steady for 11 s until Escape (run 69: 182 lifts). So the three low-field tour
    steps are **hard checks** now (in the merge, not yet run on main).
- **Very next step:**
  1. Check main's run for `c11c712` (or the HANDOFF commit after it) is green; fix or revert if red.
  2. Check `10-select-several-synced.jpg` (main's run) for the 3 s Sync toast.
  3. Then PROGRESS.md › Next 3 tasks (rotation / Split View in the tour).
- **Open problems:**
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`.
  - The artifact zip download URL (blob.core.windows.net) is blocked from the sandbox: wait for
    the `ci-artifacts` report.
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
