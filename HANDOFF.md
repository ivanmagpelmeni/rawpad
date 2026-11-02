# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-11-02, about 03:35 Berlin, by the scheduled session (02:54–03:40 Berlin,
  the ninth and last run of the night).
- **Upstream:** not checked: `storytold/lightcraft` isn't attached to this session. Last known:
  `v0.9.1` (checked 2026-11-01 19:54 Berlin).
- **Main:** green. Main run 91 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41369736165,
  the `p7-stroke-start` merge) succeeded and **released v1.91**. Only Markdown was pushed to main
  this session (no run). **Nothing merged tonight on purpose** (DECISIONS 82, in the branch).
- **Done this session:**
  - Branch run 93 failed only on the tour's label steps: even with the Edit panel closed, in
    portrait (820 pt) the bottom bar has about 205 pt between the view modes and the zoom group;
    the full pill needs 208 (236 with the label button), so the iPad shows **no stars, flags or
    label in portrait at all** (`runs/93/08-label-menu.jpg`).
  - Branch **`p7-label-pill`**, commit a19ab7bd (DECISIONS 81 and an UPSTREAM row, in the
    branch): the stars/flags code moved into `bottombar.rs › rate_and_flag`; on iOS and in unit
    tests, a bar too narrow for the full pill but at least 98 pt wide shows a **compact pill**: a
    50-pt rating button `icon:ratingMenu` (star, count, pick/reject flag) whose menu holds the
    stars and flags (`star:bottom:<n>`, `flag:<id>`), then the label button `icon:labelMenu`.
    Unit test `tests_curve::a_narrow_bottom_bar_shows_the_compact_rating_pill` (narrows the
    window with `ui.resize` until the compact pill shows; rates 4, picks, labels green). Tour:
    new shots `08-compact-pill`, `08-rating-menu`, `08-rated-3` before the label steps.
  - Branch run 94 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41375114684): unit
    tests (incl. the new one), build and clippy green; **all new tour steps passed** and the shots
    look right (`runs/94/08-rating-menu.jpg`: five stars and three flags in a menu above the
    compact pill; `08-label-menu.jpg`: the colour menu). The one failure was the tour's own
    fault: the photo already had 3 stars, so tapping star 3 cleared it (desktop toggle
    behaviour), and the later ⌘Z check (expects 3 after undoing key 5) failed.
  - Fix pushed (645a56bc): the tour taps star 4 (shot `08-rated-4`), then opens the menu again
    and taps star 3. Branch run 95 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/41378092175)
    started 02:33 UTC; not finished at session end.
- **Very next step:**
  1. Branch run 95 green (tour 0 ERR)? Look at `runs/95/08-rated-4.jpg` (the pill says 4). If the
     ⌘Z check still fails, read `replies.jsonl` around `star:bottom:3` (maybe the second menu
     didn't open: add a `{"sleep":2}`).
  2. If green and no main run is going: `git fetch origin; git checkout main; git merge --ff-only
     origin/main; git merge --no-ff origin/p7-label-pill`, push, and **watch the main run to the
     end** (about 40 minutes; it releases about 1.97). PROGRESS.md / DECISIONS.md may conflict
     (main has newer PROGRESS, the branch has DECISIONS 80–82): keep both sides.
  3. Then add to PROGRESS › Needs Ivan › This week: "in portrait, tap the small ★ button at the
     bottom to rate/flag; the circle next to it sets a colour label".
  4. Then PROGRESS.md › Next 3 tasks › 2 (hover-only reset hints, audit item 5).
- **Open problems:**
  - Returning free pages (DECISIONS 77) freed 0 MB in the simulator; only a real iPad's log
    ("memory warning: returned X MB") would tell. Resident size after zooming out stays ~550 MB.
  - Whether a 64 MB pool slows panning a zoomed raw is not measured. Watch `ui.inspect` › `perf`
    if Ivan reports slow pans.
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
    `ui.resize {"width", "height"}` resizes the headless window in unit tests.
  - `crate::i18n::tr_format!` accepts only messages already in the translations; build new text
    from existing messages or plain `tr`.
  - `date` in the sandbox runs on UTC; Berlin is UTC+1 (winter time). Shell commands time out
    after 2 minutes: wait for CI with `sleep 115` per call.
