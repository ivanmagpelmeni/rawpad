# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-26, about 01:35 Berlin, by the scheduled session (00:55–01:40 Berlin).
- **Upstream:** not re-checked (not the first run of the night).
- **Main:** `99ba506`, **green, released 1.64** (run 64,
  https://github.com/ivanmagpelmeni/rawpad/actions/runs/40243221933). Nothing merged into main
  this session (only these docs).
- **Done this session:**
  - Branch **`p4-snapshot-preview`** (`8a0e78f` + merge of main `565b1de`): the tour's slider
    long-press step opened nothing because `panel.edit` **toggles** and the Edit panel was already
    open (run 63's `11-slider-menu.jpg` showed the right panel closed). Now
    `ui.set {"view":"detail","right":"edit","openSections":["light"]}`; the slider clicks have
    `expectOk`, and the three `clickVisible` long-press steps are no longer soft (DECISIONS 65).
    **Branch run 65** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40247008832): every
    step up to the tour passed (tour included); the device build was going at handoff, the report
    lands in `ci-artifacts` `runs/65/` about 10 minutes later.
  - New branch **`p4-film-select`** (from `p4-snapshot-preview`, so it contains it): while Select
    Several is on, the loupe shows a pill above the filmstrip, "Select Several: N photos" + *Done*
    (`filmSelectSeveral`, `button:filmSelectSeveralDone`; `panels/detail.rs`
    `select_several_pill`), test
    `tests_curve::the_loupe_shows_select_several_above_the_filmstrip_and_done_ends_it`, tour shot
    `10-select-several-loupe` (detail and back to the grid between the Sync and Done steps).
    DECISIONS 64, UPSTREAM row. Run 66 failed to compile: `tr_format!` only accepts messages listed
    in the translations (a new "{n} selected" isn't); fixed in `P4: loupe pill text from existing
    messages`. **Branch run 67** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40247973429)
    was going at handoff.
- **Very next step:**
  1. Run 65: open `runs/65/11-slider-menu.jpg` (menu with *Reset Exposure* next to the Exposure
     slider), `11-version-preview.jpg` (*Bright* previewed over the reset photo),
     `11-history-preview.jpg`, `11-preset-preview.jpg`. If good, merge `origin/p4-snapshot-preview`
     into main with a merge commit and push (fetch first: CI pushes `SideStore source` commits).
     If the tour failed, read `runs/65/tour.log`.
  2. Run 67: if green and `10-select-several-loupe.jpg` shows the pill above the filmstrip, merge
     `origin/p4-film-select` into main **after main's run for step 1 is done** (pushing code to main
     while a main run goes cancels its release). If the pill's unit test failed, read
     `runs/67/ui-tests.log` (likely suspects: the `Area` position, or `ui.clickWidget` not reaching
     a Foreground area).
  3. Then PROGRESS.md › Next 3 tasks (keyboard-shrink CI check).
- **Open problems:**
  - Run 61's `10-select-several-synced.jpg` shows the strip but no toast at 0.3 s; the unit test
    proves the sync and the toast text, so the shot's timing or toast position is still unclear.
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview` (and `p4-snapshot-preview`,
    `p4-film-select` once merged).
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
