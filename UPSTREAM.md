# Upstream (storytold/lightcraft)

- **Last merged upstream commit:** `4f0b110bcbd2e350c6d97b47f1d824c2222295c4`, Release 0.9.1
  (2026-10-17). This is the fork point.
- **Last checked:** 2026-10-25 (newest upstream tag still `v0.9.1`; `git ls-remote --tags
  https://github.com/storytold/lightcraft` works from the sandbox when the releases API doesn't).
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
| `crates/engine/src/export.rs` (`apple_photos_unsupported`) | Allows iOS as well as macOS (the iPad host installs a Photos runner) | Re-apply the `any(target_os = "macos", target_os = "ios")` |
| `crates/ui-egui/src/panels/dialogs.rs` (export dialog, *After export*) | The *Add to Apple Photos* checkbox is compiled for iOS too; on iOS the album is cleared and its field and macOS note are hidden | Re-apply both `cfg` changes and the `if cfg!(target_os = "ios")` |
| `crates/ui-egui/src/menus.rs` (`reveal_label`) | Returns "Share…" on iOS (the host's `reveal` opens the share sheet) | Re-apply the `if cfg!(target_os = "ios")` branch at the top |
| `crates/ui-egui/src/panels/bottombar.rs` (`show`, before the Before/After button; `before_after_menu`) | On iOS a chevron button right of Before/After opens a menu with every Before/After view (Split, Top/Bottom, Split Top/Bottom, Show Original); ids `icon:beforeAfterModes`, `beforeAfterMode:<key>` (CI tour) | Re-apply the `if cfg!(target_os = "ios")` block and the function |
| `crates/ui-egui/src/widgets.rs` (`slider`, `touch_scrolls`) | On iOS a drag on a slider's track that sets off mostly vertically scrolls the panel and leaves the value alone | Re-apply the `else if touch_scrolls(…)` branch before the drag code, and the function |
| `crates/ui-egui/src/panels/detail.rs` (`pins_live`) | On iOS the Tool Overlay's Auto mode always shows mask and spot pins (no hover on a touch screen) | Re-apply the `cfg!(target_os = "ios") ||` at the start |
| `crates/ui-egui/src/panels/edit.rs` (`curve_editor`, `CURVE_POINT_HIT`) | On iOS a press within 22 pt (desktop 10) grabs a curve point; on iOS (and in unit tests) the curve's long-press menu starts with *Delete Point* for the inner point under the press (id `curveMenu:deletePoint`); the menu call moved below the points (and into the parametric branch) | Re-apply the constant, the `secondary_clicked` block and the two `context_menu` calls |
| `crates/ui-egui/src/panels/detail.rs` (`CROP_GRAB`, `MASK_GRAB`, crop `drag_started`) | On iOS crop handles and mask pins/handles are grabbed within 22 pt (desktop 12 / 10), and a crop drag starts at `press_origin()` | Re-apply the two constants and the `press` line |
| `crates/ui-egui/src/panels/right.rs` (`crop`, after *Reset*) | On iOS a *Done* button (`button:cropDone`) runs `tool.done` | Re-apply the `if cfg!(target_os = "ios")` button |
| `crates/ui-egui/src/panels/detail.rs` (radial mask handles, `ROT_KNOB_GAP`; `targeted_drag`) | On iOS the radial rotate knob is drawn 44 pt out from the right-edge handle, 7 pt radius; the targeted adjustment starts at `press_origin()` | Re-apply the `rot_desk`/`rot_q` lines and the `press` line |
| `crates/ui-egui/src/widgets.rs` (`stars`, `STAR_TOUCH_H`) | On iOS each star's click area is 36 pt tall and covers the gaps | Re-apply the `if cfg!(target_os = "ios")` interact block |
| `crates/ui-egui/src/panels/edit.rs` (colour wheel eye) | On iOS the wheels' show/hide eye is always drawn (no hover) and its touch area is 28 pt | Re-apply the `|| cfg!(target_os = "ios")` and the `reach` rect |
| `crates/ui-egui/src/panels/grid.rs` (`touch_multi`, `context_menu`, cell click) and `panels/detail.rs` (filmstrip click) | On iOS *Select Several* / *Done Selecting* at the top of the photo menu (`gridMenu:selectSeveral`); while on, a tap toggles the photo in the selection | Re-apply the function, the menu block, the two `|| touch_multi(…)` and `select_several_bar` (the strip under the grid title with Sync Settings and Done, DECISIONS 59, 60) |
| `crates/ui-egui/src/widgets.rs` (`slider`, `menu_reset`) and `tests_curve.rs` (`a_sliders_name_menu_resets_it`) | On iOS (and in unit tests) a long-press on a slider's name opens a menu with *Reset <name>* (`sliderMenu:reset:<control>`), DECISIONS 61 | Re-apply the `context_menu` block after `sliderLabel:` and `|| menu_reset` on the double-click reset; keep the test |
| `crates/ui-egui/src/panels/presets.rs` (preset row `context_menu`) and `tests_curve.rs` (`a_presets_long_press_menu_previews_and_applies_it`) | On iOS (and in unit tests) a preset's long-press menu previews the preset in the loupe while open and starts with *Apply* (`presetMenu:apply:<id>`), DECISIONS 62 | Re-apply the `if cfg!(target_os = "ios") \|\| cfg!(test)` block at the top of the row's `context_menu` |
| `crates/ui-egui/src/panels/right.rs` (`versions` and `activity` row `context_menu`s) and `tests_curve.rs` (`a_versions_long_press_menu_previews_and_restores_it`, `a_history_steps_long_press_menu_previews_and_goes_back_to_it`) | On iOS (and in unit tests) a version's or History step's long-press menu previews it in the loupe while open; ids `versionMenu:restore:<i>`, `historyMenu:restore:<i>` (History's menu gains a *Restore* item on iOS), DECISIONS 63 | Re-apply the two `if (cfg!(target_os = "ios") \|\| cfg!(test))` blocks at the top of the menus |
| `crates/ui-egui/src/panels/grid.rs` (`touch_multi`) | Also on in this crate's unit tests when a test sets the flag (`tests_curve::the_select_several_strips_sync_copies_settings_to_the_other_photos`) | Keep `cfg!(test)` in the condition |

### Upstream files compiled into RawPad unchanged (`#[path]` in `apps/rawpad-ios/src/ios.rs`)

| File | Used for | On merge |
|---|---|---|
| `apps/lightcraft/src/control_server.rs` | The CI control channel | Nothing to do; a changed API shows up as a build error in CI |
| `apps/lightcraft/src/logging.rs` | Log to stderr + `Documents/Logs/lightcraft.log`, panics recorded | Same |

## Merge log

| Date | Upstream | Conflicts | Notes |
|---|---|---|---|
| 2026-10-18 | 0.9.1 (`4f0b110`) | n/a | Fork point |
