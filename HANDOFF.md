# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-20, about 09:50 Berlin, by an interactive session (Ivan asked to upload the
  work the 2026-10-19 audit session couldn't push).
- **Upstream:** not checked.
- **State of the branches:**
  - **main** = this morning's session (`e773b29`): keyboard avoidance merged, main run 40
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39385342769) should release about 1.40.
    Check it first; if red, fix or `git revert -m 1 3681a8a`.
  - **`audit-2026-10-19`** = the audit session's 7 code commits (applied from the Project's
    `claude/rawpad-audit-2026-10-19.mbox` unchanged, every file's blob hash checked against the
    patches) + a merge of main + this session's doc commit. Its 8th patch (docs only) was not
    applied as is: its content went into PROGRESS.md (audit table, summaries, next tasks, Needs
    Ivan) and DECISIONS.md (renumbered **47–52**, because the morning session took 45 and 46).
    Code comments were renumbered to match (keyboard 47, gestures 49).
  - Merge conflict resolved: `ios/sim_tour.jsonl` after `04-text-field`: kept the audit's
    `checkLog` *and* the morning's typing check (`hostKeys rq` → `copied`).
- **None of the audit code has been compiled yet.** The branch's CI run is the first compile.
- **Very next step:**
  1. Find the branch's CI run: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?branch=audit-2026-10-19`.
     If there is none, trigger `ipad.yml` on ref `audit-2026-10-19` with `{"release": false}`
     (branch pushes don't start it by themselves).
  2. Fix compile / clippy / test failures on the branch. Likely spots: `egui::Popup::menu(&modes)`
     in `panels/bottombar.rs`; `ui.scroll_with_delta` needing `&mut Ui` in `widgets.rs`
     `touch_scrolls` (it takes `&Ui`); `let … && …` chains (edition 2024) in `touch.rs` / `ios.rs`;
     unused-import warnings; the unit tests in `keys.rs` / `touch.rs`.
  3. Look at `runs/<n>/07-slider-no-keyboard`, `07-brush-no-keyboard` (no keyboard),
     `04-text-field` and `04-caption-keyboard` (keyboard up), `07-slider-vertical-drag` (mask
     Exposure unchanged), `08-before-after-menu`, `08-split`, `08-top-bottom`,
     `08-split-top-bottom`, `08-original`, `08-side-by-side`; and the tour log's `checkLog` lines.
  4. Only then merge `audit-2026-10-19` into main (a release follows) and put the real version
     number into PROGRESS.md › Editing audit and Needs Ivan.
- **Open problems:**
  - Ivan's "split doesn't work": cause not confirmed. Code showed the split views were reachable
    only by keyboard or Menu › View; if the new menu works in CI but Ivan still sees a problem,
    look at the `Slot::Before` render in the device log.
  - Keyboard avoidance (DECISIONS.md 45) finds the focused field with
    `egui_wants_keyboard_input()`; with decision 47 the keyboard only comes up for a tapped text
    box, so a focused slider can't trigger it, but check `04-caption-keyboard` still shows the
    keyboard and the log still says `UI shrunk by`.
  - The shrink path itself isn't exercised in CI yet (both fields sit above the keyboard).
  - Real-device checks (Pencil, fingers, two-finger tap) need Ivan.
  - Audit backlog: PROGRESS.md › Next 3 tasks and the queue.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`;
    steps: `…/actions/runs/<id>/jobs` (public, 60 requests/hour). Releases: `…/releases?per_page=2`.
  - Reports: `git fetch --depth 1 origin ci-artifacts`; `git show FETCH_HEAD:LATEST`; files under
    `runs/<n>/` (`tour.log`, `replies.jsonl`, `app-stderr.log`, `build-sim.log`, screenshots).
  - Pushing code to main while a run is going cancels it. Markdown-only pushes don't start a run.
  - Branch runs: `workflow_dispatch` of `ipad.yml` on the branch with `{"release": false}`; they
    don't cancel main runs. A full run takes 25–30 minutes; the tour result is known after about 17.
