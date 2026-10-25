# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-25, about 19:35 Berlin, by the scheduled evening session (18:55–19:40).
- **Upstream:** checked; newest tag still `v0.9.1` (the fork point). The releases API needs repo
  access, `git ls-remote --tags https://github.com/storytold/lightcraft` works.
- **Done:**
  - Main was green before this session: run 44 released **1.44** (the editing audit).
  - Touch grab radius 22 pt on iPad for curve points, crop handles, mask pins/handles; a crop
    drag starts at the press (DECISIONS.md 53). Long-press a curve point → *Delete Point* (54).
    A *Done* button in the Crop panel on iPad, tapped in the CI tour (`09-crop-*` shots) (56).
    CI also runs `cargo test -p lightcraft-ui-egui --lib tests_curve` (55; it passed, with the
    new `the_curve_menu_deletes_the_point_under_the_press`).
  - Branch run 45 (`p4-curve-touch`, curve only,
    https://github.com/ivanmagpelmeni/rawpad/actions/runs/40218995583): tour green.
  - Branch run 46 (`p4-touch-all`, all code,
    https://github.com/ivanmagpelmeni/rawpad/actions/runs/40219287994): compile, clippy, unit
    tests and **tour green** (the tour's `button:cropDone` click succeeded). Its screenshots were
    **not looked at yet** (the report is published after the .ipa build, after this session).
  - Merged into main (about 19:35) → the main run after it should release about 1.48.
- **Very next step:**
  1. Check that main run: green and released? If red, fix or `git revert -m 1 <merge>`.
  2. Look at `09-crop-done-button.jpg` (Done button visible next to Reset) and
     `09-crop-applied.jpg` (back in Edit, no crop frame) in the newest `runs/<n>/` on
     `ci-artifacts`. If wrong, fix.
  3. Then PROGRESS.md › Next 3 tasks #2 (radial rotate knob, rating stars, targeted adjustment
     at the press).
- **Open problems:**
  - FEEDBACK.md's two items (split, keyboard) wait for Ivan to re-test on 1.44 or later.
  - The keyboard-shrink path is still only unit-tested (run 43's log: UI shrank by 0 pt).
  - Real-device checks (Pencil, fingers, long-press, two-finger tap) need Ivan.
  - Branches `audit-2026-10-19`, `p4-curve-touch`, `p4-touch-all` are merged; delete them on
    GitHub when convenient (no tool for it here).
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `build-sim.log`, `ui-tests.log`,
    screenshots).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`
    (GitHub MCP `actions_run_trigger`); runs on one branch cancel each other, so use a second
    branch to test two states at once. A full run takes 25–35 minutes; the tour result is known
    after about 20 (the new ui-egui test step adds ~5 minutes on a cold cache).
