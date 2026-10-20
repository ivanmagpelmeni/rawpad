# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-20, about 11:00 Berlin, by an interactive session (Ivan asked to upload the
  work the 2026-10-19 audit session couldn't push).
- **Upstream:** not checked.
- **Done:**
  - Applied the 2026-10-19 audit session's 7 code commits from the Project's
    `claude/rawpad-audit-2026-10-19.mbox` unchanged (blob hashes checked) on branch
    `audit-2026-10-19`, merged main (morning keyboard avoidance) into it, folded its 8th (docs)
    patch into the docs by hand (decisions renumbered 47–52).
  - Branch run 41 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39388400770): compiled,
    clippy and unit tests green first time; tour failed only on the search box keyboard
    (`checkLog … present since search`). Fix 1 (`keys::tap_age`, 4 frames): run 42
    (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39391250312) still failed; its new log
    line showed `PlatformOutput::ime.rect` is the text cursor (zero wide). Fix 2: the focused
    widget's rect. Run 43 (https://github.com/ivanmagpelmeni/rawpad/actions/runs/39393539938)
    **all green**; screenshots looked at: `04-text-field` keyboard up, `07-slider-no-keyboard`
    and `07-brush-no-keyboard` none, `07-slider-vertical-drag` Exposure unchanged (−0.80), mask
    pins visible, `08-*` every Before/After view correct (split lines visible).
  - Merged `audit-2026-10-19` into main and pushed (about 11:00); the main run after it should
    release about 1.44.
- **Very next step:**
  1. Check that main run: green and released? If red, fix or revert the merge commit
     (`git revert -m 1 <merge>`).
  2. Then PROGRESS.md › Next 3 tasks #2 (touch targets and grab position).
- **Open problems:**
  - Ivan's "split doesn't work": cause not confirmed. Code showed the split views were reachable
    only by keyboard or Menu › View; if the new menu works in CI but Ivan still sees a problem,
    look at the `Slot::Before` render in the device log.
  - Run 43's log: the keyboard covers 257 pt and the UI shrank by 0 pt for both the search box
    and the caption (both sit above it), so the shrink path is still only unit-tested.
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
