# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-02, about 02:30 Berlin, by the scheduled session (01:54–02:40 Berlin,
  the eighth run of the night).
- **Upstream:** not checked: `storytold/lightcraft` isn't attached to this session. Last known:
  `v0.9.1` (checked 2026-11-01 19:54 Berlin).
- **Main:** main run 89 green, released 1.89. **`p7-stroke-start` merged** (e3c8d6be); main run
  91 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41369736165) started 00:55 UTC and
  releases about 1.91: at 01:31 UTC its unit tests, clippy, the whole tour and the
  device build were green; only the release step was still to come.
- **Done this session:**
  - Branch **`p7-label-pill`** (DECISIONS 80, UPSTREAM row): on iOS and in unit tests the bottom
    bar's star/flag pill ends with a colour-label button (`icon:labelMenu`) opening the label menu
    (`bottomLabel:<colour>` / `bottomLabel:none`); unit test
    `tests_curve::the_bottom_bars_label_button_sets_and_clears_the_colour_label`; tour shots
    `08-label-menu`, `08-label-green`.
  - Branch run 92 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41369868252): unit tests
    (incl. the new one), build, clippy green; **tour failed only on the 4 new label steps**: in
    portrait (820 pt) with the Edit panel open the bottom bar has no room for the pill at all (no
    stars or flags there either, before this change too; see `runs/92/08-label-menu.jpg`).
  - Fix pushed (843ce5b7, "tour closes the Edit panel before the label button steps"): `ui.set
    {"right":"none"}` before, `{"right":"edit"}` after. Branch run 93
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41372532845) started 01:26 UTC.
- **Very next step:**
  1. Main run 91 green? (If red: read its log; fix or revert the merge e3c8d6be.)
  2. Branch run 93: tour 0 ERR? Look at `runs/93/08-label-menu.jpg` (menu with five colours
     above the pill) and `08-label-green.jpg` (green dot at the pill's end). If the label button
     is still missing, the bar is too narrow even without the panel: read `replies.jsonl` /
     `ui.widgets`, maybe turn the phone landscape in the tour or shrink the pill.
  3. If green and no main run is going: `git fetch origin; git checkout main; git merge --ff-only
     origin/main; git merge --no-ff origin/p7-label-pill`, push.
  4. Then PROGRESS.md › Next 3 tasks › 2 (a compact pill when the bar is narrow).
- **Open problems:**
  - In portrait with a right panel open, the bottom bar shows no stars, flags or label (no room
    for the pill). Pre-existing; noted in PROGRESS › Next 3 tasks › 2.
  - Returning free pages (DECISIONS 77) freed 0 MB in the simulator; only a real iPad's log
    ("memory warning: returned X MB") would tell. Resident size after zooming out stays ~550 MB.
  - Whether a 64 MB pool slows panning a zoomed raw is not measured (the tour has no pan timing
    on the raw). Watch `ui.inspect` › `perf` if Ivan reports slow pans.
  - Split View / Stage Manager resizing can't be scripted in the simulator: on Ivan's list
    (PROGRESS.md › Needs Ivan › item 8).
  - `ui.inspect` › `canvasRect` in the grid includes the title row, so the tour hard-codes `top`.
  - FEEDBACK.md's two items wait for Ivan to re-test (PROGRESS.md › Needs Ivan › This week).
  - Real-device checks (Pencil, finger pinch, long-press) need Ivan.
  - Add to Apple Photos in the simulator stays flaky (soft check).
  - `apps/rawpad-ios/Cargo.toml` still lists `lightcraft-sysmem` (now unused on iOS); removing it
    changes Cargo.lock, which this sandbox can't regenerate: leave it.
  - Merged branches to delete on GitHub some time: `audit-2026-10-19`, `p4-curve-touch`,
    `p4-touch-all`, `p4-touch-2`, `p4-select-several`, `p4-select-bar`, `p4-select-sync`,
    `p4-reset-menu`, `p4-sync-test`, `p4-preset-preview`, `p4-snapshot-preview`,
    `p4-film-select`, `p4-keyboard-check`, `p4-rotate`, `p4-raw24`, `p6-memtrim`, `p6-relaunch`,
    `p6-hard-checks`, `p6-preview-cap`, `p6-gpu-pool`, `p7-stroke-start`.
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
    `git reset --hard` (and a chained fetch/checkout/reset) is refused in the sandbox: use
    `git checkout main` then `git merge --ff-only origin/main`.
    `git fetch origin main` overwrites FETCH_HEAD: fetch `ci-artifacts` again before `git show`.
  - A memory step's numbers: `runs/<n>/memory.jsonl` (one JSON line per `memory` tour step).
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`, owner `ivanmagpelmeni`). A full run takes 25–35 minutes;
    two branch runs can go at once, but a run triggered on a branch that already has one queued
    replaces it (concurrency group per ref). A push to a branch doesn't start a run, so push every commit
    before triggering. CI only runs the `tests_curve` module of the UI crate's unit tests, so
    app-level tests for iPad patches go there.
  - `ui.set` view names: `photoGrid`, `squareGrid`, `detail`, `compare`, `survey`, `reference`,
    `people`. Panels: `panel.edit`, `panel.presets` (toggle!), `panel.versions`, `panel.activity`;
    to open a panel for sure use `ui.set {"right": "edit"}` (also `openSections`).
  - `crate::i18n::tr_format!` accepts only messages already in the translations; build new text
    from existing messages or plain `tr`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time). Shell commands time out
    after 2 minutes: wait for CI with `sleep 115` per call.
