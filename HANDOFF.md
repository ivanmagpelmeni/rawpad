# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, about 15:10 Berlin, at the end of the kickoff session (interactive).
- **State:** main is green (CI run 8, `85b8e7b`). Release **1.8** is published and listed in
  `sidestore/source.json`.
- **Works in the simulator:**
  - launch, demo library, grid, Develop, sliders, radial mask
  - JPEG export to `Documents/Exports`
  - Files picker code (compiles; can't be driven in CI because it's system UI)
- **In progress:** nothing half-done. Phase 3 (Ivan's first install) is waiting on Ivan.
- **Very next step:** `PROGRESS.md` → *Next 3 tasks*, item 1 (long-press → secondary click),
  unless `FEEDBACK.md` has items. Feedback comes first.
- **How to check CI:**
  - Runs: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`
    (public, 60 requests/hour; don't poll faster than once a minute).
  - Reports: clone or fetch the `ci-artifacts` branch with git
    (`git fetch --depth 1 origin ci-artifacts`). `LATEST` names the run; `runs/<n>/` holds
    `run.json`, `build-sim.log`, `clippy.log`, `tour.log`, `app-stderr.log`, and `*.jpg`
    screenshots.
  - `raw.githubusercontent.com` and release downloads are blocked by the sandbox proxy; git and
    `api.github.com` work.
  - Watching for a new report: poll `git ls-remote origin ci-artifacts` every 60 s.
  - Pushing again while a run is going cancels it (concurrency). Its report then says
    `cancelled`.
- **Build times:** about 8 minutes cold for the sim build, plus the tour and the device build;
  with the cache, about 15–20 minutes per full run. Batch changes before pushing.
- **Open problems:**
  - The AI mask buttons show on iPad (models are excluded).
  - The bottom toolbar sits near the home indicator.
  - No Photos-app export yet.
  - UI prefs aren't saved between launches.
