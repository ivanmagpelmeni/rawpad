# Upstream (storytold/lightcraft)

- **Last merged upstream commit:** on `main`: `16636c45`, Release 0.19.0 (2026-11-06), merged
  2026-11-08 from branch `upstream-0.19.0` with v0.9.1 as the explicit base (DECISIONS 83). From
  now on a normal `git merge <tag>` works for later releases.
- **Last checked:** 2026-11-08 (newest upstream tag `v0.19.0`). `git ls-remote --tags
  https://github.com/storytold/lightcraft | sort -V` works from the sandbox when the releases API
  doesn't. **Use `sort -V`:** plain `sort` puts v0.9.1 after v0.19.0 (that hid ten releases).
- **Paths:** the patch table names the 0.19.0 files (updated 2026-11-08). Every patch carries a
  `RawPad` comment: `grep -rn RawPad crates/`. iPad UI tests live in our own `tests/ipad.rs`.
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
| `crates/engine/src/export/params.rs` (`apple_photos_unsupported`) | Allows iOS as well as macOS (the iPad host installs a Photos runner) | Re-apply the `any(target_os = "macos", target_os = "ios")` |
| `crates/ui-egui/src/dialogs.rs` (export dialog, *After export*, `export_apple_photos_rows`) | The *Add to Apple Photos* checkbox is compiled for iOS too; on iOS the album is cleared and its field and macOS note are hidden | Re-apply both `cfg` changes and the `if cfg!(target_os = "ios")` |
| `crates/ui-egui/src/menus/files.rs` (`reveal_label`) | Returns "Share…" on iOS (the host's `reveal` opens the share sheet) | Re-apply the `if cfg!(target_os = "ios")` branch at the top |
| `crates/ui-egui/src/panels/bottombar.rs` (`show`, before the Before/After button; `before_after_menu`) | On iOS a chevron button right of Before/After opens a menu with every Before/After view (Split, Top/Bottom, Split Top/Bottom, Show Original); ids `icon:beforeAfterModes`, `beforeAfterMode:<key>` (CI tour) | Re-apply the `if cfg!(target_os = "ios")` block and the function |
| `crates/ui-egui/src/widgets.rs` (`slider`, `touch_scrolls`) | On iOS a drag on a slider's track that sets off mostly vertically scrolls the panel and leaves the value alone | Re-apply the `else if touch_scrolls(…)` branch before the drag code, and the function |
| `crates/ui-egui/src/panels/detail/masks.rs` (`pins_live`) | On iOS the Tool Overlay's Auto mode always shows mask and spot pins (no hover on a touch screen) | Re-apply the `cfg!(target_os = "ios") ||` at the start |
| `crates/ui-egui/src/panels/edit/curve.rs` (`curve_editor`, `CURVE_POINT_HIT`) | On iOS a press within 22 pt (desktop 10) grabs a curve point; on iOS (and in unit tests) the curve's long-press menu starts with *Delete Point* for the inner point under the press (id `curveMenu:deletePoint`); the menu call moved below the points (and into the parametric branch) | Re-apply the constant, the `secondary_clicked` block and the two `context_menu` calls |
| `crates/ui-egui/src/panels/edit/curve.rs` (channel buttons, after the `double_clicked` reset) and `panels/edit/grading.rs` (`grading_wheel`, `menu_reset`) | On iOS (and in unit tests) a long-press on a curve channel button opens `curve_reset_menu` for it; a long-press on a colour wheel opens *Reset <wheel>* (`wheelMenu:reset:<key>`), DECISIONS 90 | Re-apply the `context_menu` call and the wheel's menu block with `\|\| menu_reset` on the double-click reset |
| `crates/ui-egui/src/panels/detail/crop.rs` (`CROP_GRAB`, crop `drag_started`) and `panels/detail/masks.rs` (`MASK_GRAB`) | On iOS crop handles and mask pins/handles are grabbed within 22 pt (desktop 12 / 10), and a crop drag starts at `press_origin()` | Re-apply the two constants and the `press` line |
| `crates/ui-egui/src/panels/right/crop.rs` (after *Reset*) | On iOS a *Done* button (`button:cropDone`) runs `tool.done` | Re-apply the `if cfg!(target_os = "ios")` button |
| `crates/ui-egui/src/panels/detail/masks.rs` (radial mask handles, `ROT_KNOB_GAP`) and `panels/detail.rs` (`targeted_drag`) | On iOS the radial rotate knob is drawn 44 pt out from the right-edge handle, 7 pt radius; the targeted adjustment starts at `press_origin()` | Re-apply the `rot_desk`/`rot_q` lines and the `press` line |
| `crates/ui-egui/src/panels/detail/masks.rs` (`stroke_lead_in`, brush stroke start), `panels/detail/retouch.rs` (Remove stroke start) and `tests/ipad.rs` (`a_brush_or_spot_stroke_starts_where_the_finger_touched`) | On iOS (and in unit tests) a brush or spot stroke that starts with a drag begins at `press_origin()`, not where the drag passed egui's threshold, DECISIONS 79 | Re-add the function and its two calls in the new-gesture branches |
| `crates/ui-egui/src/panels/bottombar.rs` (`LABEL_W`, `touch_label`, `label_button`, `centre`) and `tests/ipad.rs` (`the_bottom_bars_label_button_sets_and_clears_the_colour_label`) | On iOS (and in unit tests) the rating pill ends with a colour-label button and menu (`icon:labelMenu`, `bottomLabel:<colour>`), DECISIONS 80 | Re-add the constant and two functions; in `centre` re-apply `with_label` / `pill_w` in place of `PILL_W` and the call after the flags |
| `crates/ui-egui/src/panels/bottombar.rs` (`rate_and_flag`, `COMPACT_W`, `compact_pill`, `centre`) and `tests/ipad.rs` (`a_narrow_bottom_bar_shows_the_compact_rating_pill`) | Stars and flags moved into `rate_and_flag`; on iOS (and in unit tests) a bar too narrow for the full pill shows a compact pill (`icon:ratingMenu` + label button), DECISIONS 81 | Re-extract the stars/flags loop into the helper; in `centre` re-apply the `compact_pill` call before the early return |
| `crates/ui-egui/src/startup.rs` (`open_at_startup`) | On iOS no first-run Welcome or What's New dialog, DECISIONS 87 | Re-apply `\|\| cfg!(target_os = "ios")` in the first `if` |
| `crates/ui-egui/src/dialogs/updates_ui.rs` (`check`) and `menus/menubar.rs` (`app.checkForUpdates` enabled) | On iOS the built-in updater is off (RawPad updates through SideStore) | Re-apply the `if cfg!(target_os = "ios")` return and the `!cfg!` condition |
| `crates/ui-egui/src/widgets/rating.rs` (`stars`, `STAR_TOUCH_H`) | On iOS each star's click area is 36 pt tall and covers the gaps | Re-apply the `if cfg!(target_os = "ios")` interact block |
| `crates/ui-egui/src/panels/edit/grading.rs` (colour wheel eye) | On iOS the wheels' show/hide eye is always drawn (no hover) and its touch area is 28 pt | Re-apply the `|| cfg!(target_os = "ios")` and the `reach` rect |
| `crates/ui-egui/src/panels/grid.rs` (`touch_multi`, `context_menu`, cell click) and `panels/detail/filmstrip.rs` (filmstrip click) | On iOS *Select Several* / *Done Selecting* at the top of the photo menu (`gridMenu:selectSeveral`); while on, a tap toggles the photo in the selection | Re-apply the function, the menu block, the two `|| touch_multi(…)` and `select_several_bar` (the strip under the grid title with Sync Settings and Done, DECISIONS 59, 60; its Sync toast lasts 3 s, DECISIONS 66) |
| `crates/ui-egui/src/widgets.rs` (`slider`, `menu_reset`) and `tests/ipad.rs` (`a_sliders_name_menu_resets_it`) | On iOS (and in unit tests) a long-press on a slider's name opens a menu with *Reset <name>* (`sliderMenu:reset:<control>`), DECISIONS 61 | Re-apply the `context_menu` block after `sliderLabel:` and `|| menu_reset` on the double-click reset; keep the test |
| `crates/ui-egui/src/panels/presets.rs` (preset row `context_menu`) and `tests/ipad.rs` (`a_presets_long_press_menu_previews_and_applies_it`) | On iOS (and in unit tests) a preset's long-press menu previews the preset in the loupe while open and starts with *Apply* (`presetMenu:apply:<id>`), DECISIONS 62 | Re-apply the `if cfg!(target_os = "ios") \|\| cfg!(test)` block at the top of the row's `context_menu` |
| `crates/ui-egui/src/panels/right/history.rs` (`versions` and `activity` row `context_menu`s) and `tests/ipad.rs` (`a_versions_long_press_menu_previews_and_restores_it`, `a_history_steps_long_press_menu_previews_and_goes_back_to_it`) | On iOS (and in unit tests) a version's or History step's long-press menu previews it in the loupe while open; ids `versionMenu:restore:<i>`, `historyMenu:restore:<i>` (History's menu gains a *Restore* item on iOS), DECISIONS 63 | Re-apply the two `if (cfg!(target_os = "ios") \|\| cfg!(test))` blocks at the top of the menus |
| `crates/ui-egui/src/panels/detail/filmstrip.rs` (`select_several_pill`, called at the end of `filmstrip`) and `tests/ipad.rs` (`the_loupe_shows_select_several_above_the_filmstrip_and_done_ends_it`) | While Select Several is on (iOS / tests only, `grid::touch_multi`), a pill above the filmstrip with the count and Done, DECISIONS 64 | Re-add the function and its one call |
| `crates/ui-egui/src/widgets/text_field.rs` (`KEEP_IN_VIEW`, `keep_in_view`, called after `ui.add(edit)`) (since 0.19.0 the Info panel's GPS box is a `TextField` too, so no separate call) | While the iPad host sets the temp flag (a moment after lifting the UI above the keyboard), a focused field not fully in view scrolls itself into view, DECISIONS 67 | Re-add the constant, the function and its call in `TextField` |
| `crates/ui-egui/src/panels/grid.rs` (`touch_multi`) | Also on in this crate's unit tests when a test sets the flag (`tests::ipad::the_select_several_strips_sync_copies_settings_to_the_other_photos`) | Keep `cfg!(test)` in the condition |

### Upstream files compiled into RawPad unchanged (`#[path]` in `apps/rawpad-ios/src/ios.rs`)

| File | Used for | On merge |
|---|---|---|
| `apps/lightcraft/src/control_server.rs` | The CI control channel | Nothing to do; a changed API shows up as a build error in CI |
| `apps/lightcraft/src/logging.rs` | Log to stderr + `Documents/Logs/lightcraft.log`, panics recorded | Same |

## Merge log

| Date | Upstream | Conflicts | Notes |
|---|---|---|---|
| 2026-10-18 | 0.9.1 (`4f0b110`) | n/a | Fork point |
| 2026-11-08 | 0.19.0 (`16636c45`) | 21 files: 9 workflows deleted, CLAUDE.md, ATTRIBUTION, .gitignore, 13 patched UI/engine files (upstream taken, patches re-applied) | No common history: explicit base v0.9.1 (DECISIONS 83); egui 0.37 fork; on main after branch run 103 (0 tour ERR) |
