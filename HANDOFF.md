# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, about 19:25 Berlin, end of nightly session 1 (unattended, started 18:54).
- **Upstream:** checked `git ls-remote https://github.com/storytold/lightcraft 'refs/tags/*'`:
  newest tag is still `v0.9.1` (our fork point). Nothing to merge. (The releases API for
  storytold/lightcraft is blocked in this sandbox; `git ls-remote` on tags works.)
- **Done this session:**
  - `ee9da55` P4: long-press → secondary click (`apps/rawpad-ios/src/touch.rs`), 5 unit tests,
    new CI step *Unit tests (iOS crate, on the macOS host)*. **CI run 9 green, released as
    RawPad 1.9** (`c6fbf5f` updated the SideStore source).
  - `fae8a97` P4: UI inside UIKit's `safeAreaInsets` (`native::safe_area`) + interface settings
    saved to `Library/Application Support/RawPad UI.json` (`src/prefs.rs`).
  - `e83191e` P7: AI mask tiles hidden on iOS (first upstream patch, logged in `UPSTREAM.md`);
    `DECISIONS.md` 20–23.
- **In progress:** CI run 10 for `e83191e` (the two commits above) was started at about 19:21
  Berlin and had not finished when this was written. **Check it first:**
  - green → look at `runs/10/03-masking.jpg` (5 tiles: Brush, Linear, Radial, Luminance, Color)
    and `02-develop.jpg` (bottom toolbar clear of the home indicator), and grep
    `runs/10/app-stderr.log` for `safe area (points)`. If the toolbar still touches the bottom,
    the inset read returned zeros: log says so; try `window.safeAreaInsets` of the UIWindow
    (`view.window`) instead of the view, or read after the first layout.
  - red → read `build-sim.log` / `clippy.log`. Likely suspects: the `msg_send!` returning
    `EdgeInsets` (objc2 encoding check: struct name must be `UIEdgeInsets`), the
    `lightcraft_ui_egui::docking::Workspace` path in `prefs.rs`, or a clippy lint on the
    `cfg!` filter in `masking.rs` (clippy only lints `rawpad-ios`, so unlikely). Fix or revert
    `fae8a97`/`e83191e`; never leave main red.
- **Very next step after that:** `PROGRESS.md` → *Next 3 tasks* (FEEDBACK.md is still empty).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour).
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show origin/ci-artifacts:LATEST`;
    files under `runs/<n>/` (now also `unit-tests.log`). Extract a screenshot with
    `git show origin/ci-artifacts:runs/<n>/02-develop.jpg > <scratchpad>/x.jpg` and Read it.
  - Pushing code while a run is going cancels it. Markdown-only pushes don't start a run.
  - A full run takes about 20–25 minutes (the device build is the slow part). To get a release
    out, wait for the run to finish before pushing more code.
- **Open problems:**
  - Long-press is untested on a real touchscreen (the control channel bypasses the filter by
    design). Ask Ivan via *Needs Ivan*.
  - AI items still appear in mask context menus (Add/Subtract → Sky/Subject…), and the ArtCraft
    Discord/website links show.
  - No Photos-app export or share sheet yet; no on-device log file.
