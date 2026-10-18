# Handoff

The handoff between sessions. Each session overwrites this about every 45 minutes and at its
end. The next session reads it **first**.

- **Written:** 2026-10-18, kickoff session (interactive, with Ivan).
- **State:** scaffolding pushed. The iOS host crate is written, and CI is the first compiler.
- **In progress:** getting `.github/workflows/ipad.yml` green.
- **Very next step:**
  1. Check the latest iPad workflow run: `https://api.github.com/repos/Ivanmagpelmeni/rawpad/actions/runs?per_page=3`
     (public, no login).
  2. Then read its report at
     `https://raw.githubusercontent.com/Ivanmagpelmeni/rawpad/ci-artifacts/runs/<run_number>/`.
     The files there are `build-sim.log`, `clippy.log`, `tour.log`, `app-stderr.log`,
     `run.json`, and the `*.jpg` screenshots. `LATEST` names the newest run.
  3. Fix, push, repeat.
- **Open problems:**
  - If no run shows up, Actions may still be disabled on the fork (see `PROGRESS.md` →
    *Needs Ivan*).
  - Expect first-compile API mismatches in `apps/rawpad-ios/src/ios.rs`. The egui-wgpu 0.36 /
    egui-winit 0.36 / winit 0.30 signatures were checked against egui's 0.36.2 sources, but never
    compiled.
- **How to check CI from a Claude session:**
  - `gh` is not logged in. `api.github.com` works without a login for public data (60
    requests/hour).
  - Job logs and artifacts need a login, so read the `ci-artifacts` branch instead.
