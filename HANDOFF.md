# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-26, about 02:38 Berlin, by the scheduled session (01:55–02:40 Berlin).
- **Upstream:** not re-checked (not the first run of the night).
- **Main:** run 68 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40250402919) green,
  **released 1.68** (version / History preview by long-press). Then this session **merged
  `p4-film-select`** (`1a5563f`, branch run 67 green, its `10-select-several-loupe.jpg` shows
  "Select Several: 3 photos | Done" above the filmstrip). Main's run for that merge releases about
  1.70 or later.
- **Done this session:**
  - Checked run 67's shots: the loupe pill is right. The Sync toast is still missing from
    `10-select-several-synced.jpg` (toast lasted 1.4 s; the `simctl` shot is slow).
  - New branch **`p4-keyboard-check`** (from `p4-film-select`, `cebab98`), DECISIONS 66:
    - `apps/rawpad-ios/src/ios.rs`: logs "keyboard: lifting the UI above the on-screen keyboard
      (field bottom …, keyboard top …)" when the UI shrinks; the "covers … UI shrunk by …" line is
      logged again when the shrink changes (it was keyed on the height only).
    - Tour: after the caption step, taps the first `field:` fully on screen between 905 and
      1150 pt (below the keyboard's top, about 903 pt; likely Location / City in the Info panel),
      shot `04-low-field-keyboard`, soft `checkLog` for the new line.
    - `panels/grid.rs`: the strip's "Sync Settings: N photos" toast lasts 3 s; tour sleeps 0.5 s.
    - **Branch run 69** (https://github.com/ivanmagpelmeni/rawpad/actions/runs/40251879852)
      **failed** (do not merge the branch yet). Report in `ci-artifacts` `runs/69/`.
- **Run 69 found a real keyboard-avoidance bug (likely on the iPad too):** tapping
  `field:location` (bottom 986 pt, keyboard top 898 pt) makes the UI shrink, then the keyboard
  goes down, comes back, shrinks again, about every 150 ms (`app-stderr.log` lines 40–90+: 182
  "lifting the UI" lines, "first responder for text field taken" / "for keys taken" alternating).
  The shot `04-low-field-keyboard.jpg` caught a "down" frame: Location focused, no keyboard.
  Likely cause: after the shrink the field lies below the new screen bottom, the Info panel's
  scroll area doesn't bring it into view, so egui sends no IME output → `self.text_field` is None
  (`ios.rs` line ~359) → `keys::soft_keyboard` drops the keyboard → the UI grows back → the field
  is drawn again and the recent tap brings the keyboard back. Fix ideas (pick one, in our crate):
  (a) keep the keyboard while egui still has a text field focused and `egui_wants_keyboard_input`
  (`was_up`), even with no IME rect for a frame; and/or (b) after shrinking, scroll the focused
  field into view (egui `Context::frame_state`/`scroll_to_rect` on the focused response next
  frame). Then the run should show the keyboard and Location above it, and "lifting" logged once.
- **Run 69's other failure:** `checkLog 'first responder for text field' present since search`
  saw an empty log since the mark, though the app did log it (01:18:41). Passed in run 67: look at
  how `log_text` / `logMark` offsets work in `ios/sim_test.py` if it repeats.
- **Very next step:**
  1. Check main's run 70 (`p4-film-select` merge, https://github.com/ivanmagpelmeni/rawpad/actions/runs/40253633287) is green (fix or revert if red).
  2. On `p4-keyboard-check`: fix the keyboard flicker above (add a unit test in `keys.rs` or
     `keyboard.rs` for the chosen rule), push, trigger a branch run. When
     `04-low-field-keyboard.jpg` shows the keyboard with Location above it and "lifting" is logged
     once: make the two steps hard (drop `"soft"`), check `10-select-several-synced.jpg` for the
     toast, and merge into main **after** main's current run is done.
  3. Then PROGRESS.md › Next 3 tasks (rotation / Split View in the tour).
- **Open problems:**
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test on 1.44 or later.
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select` (and `p4-keyboard-check` once merged).
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
