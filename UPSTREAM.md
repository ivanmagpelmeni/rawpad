# Upstream (storytold/lightcraft)

- **Last merged upstream commit:** `4f0b110bcbd2e350c6d97b47f1d824c2222295c4`, Release 0.9.1
  (2026-10-17). This is the fork point.
- **Merge policy:** merge each upstream release tag, at most once a week (decided 2026-10-18). To
  merge:
  1. `git remote add upstream https://github.com/storytold/lightcraft` (once).
  2. `git fetch upstream --tags`.
  3. `git merge <tag>`.
  4. Resolve conflicts using the list below.
  5. Push. CI must pass before the next release.

## Local changes to upstream files (keep this list tiny)

| File | Change | On merge |
|---|---|---|
| `docs/brand/` | Deleted (ArtCraft trademarks; forks must remove them) | Keep deleted |
| `README.md`, `README.zh-hans.md` | Logo blocks removed; fork note at the top; trademark paragraph replaced | Re-apply the same edits |
| `NOTICE` | Trademark paragraph says the marks were removed | Keep ours |
| `assets/ATTRIBUTION.md` | `docs/brand/` row replaced by the `ios/icon/` row | Keep ours plus upstream's new rows |
| `CLAUDE.md` | RawPad section prepended (points agents to `IPAD.md`) | Keep our section at the top |
| `.github/workflows/{freebsd,nix,packaging-lint,release,windows-arm64}.yml` | Deleted (desktop packaging; they would run on every upstream merge) | Keep deleted |

### Patched upstream crate source (all behind `cfg!(target_os = "ios")`)

| File | Change | On merge |
|---|---|---|
| `crates/ui-egui/src/panels/masking.rs` (`show`, after the `tiles` array) | Filters the AI tiles (Object, Describe, Subject, Sky, Background) out of *Create New Mask* on iOS | Re-apply after the `tiles` array if upstream changes it |
| `crates/ui-egui/src/panels/masking.rs` (`component_menu`) | On iOS (`ai = false`) skips Object, Describe…, Sky and Subject in the Add/Subtract/Intersect menus | Re-apply: wrap the Object/Describe buttons in `if ai`, skip `sky`/`subject` in the loop |

### Upstream files compiled into RawPad unchanged (`#[path]` in `apps/rawpad-ios/src/ios.rs`)

| File | Used for | On merge |
|---|---|---|
| `apps/lightcraft/src/control_server.rs` | The CI control channel | Nothing to do; a changed API shows up as a build error in CI |
| `apps/lightcraft/src/logging.rs` | Log to stderr + `Documents/Logs/lightcraft.log`, panics recorded | Same |

## Merge log

| Date | Upstream | Conflicts | Notes |
|---|---|---|---|
| 2026-10-18 | 0.9.1 (`4f0b110`) | n/a | Fork point |
