//! RawPad: headless tests of the iPad patches to the upstream UI (UPSTREAM.md). CI runs this module
//! (`cargo test -p lightcraft-ui-egui --lib tests::ipad`). Its own file, so upstream merges leave it alone.

use std::time::Duration;

use serde_json::json;

use crate::headless::Headless;
use crate::{LightcraftApp, Services};

const T: Duration = Duration::from_secs(20);

/// Detail view with the Edit panel's Curve flyout open on `channel`.
fn curve_open(channel: &str) -> Headless {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.app.ui.right = crate::state::RightPanel::Edit;
    if !h.app.ui.section_open("light") {
        h.app.ui.toggle_section("light");
    }
    if !h.app.ui.flyout_open("curve") {
        h.app.ui.toggle_flyout("curve");
    }
    h.app.ui.curve_channel = channel.into();
    h.step();
    h.step();
    h
}

fn exec(h: &mut Headless, command: &str, params: serde_json::Value) -> serde_json::Value {
    let r = h.request("engine.execute", json!({"command": command, "params": params}), T);
    assert_eq!(r["ok"], true, "{command}: {r}");
    r["result"].clone()
}

fn develop(h: &Headless) -> lightcraft_develop::DevelopSettings {
    let id = h.app.session.active().expect("active photo");
    (*h.app.session.develop_of(id).unwrap_or_default()).clone()
}

fn pts(v: &[lightcraft_geom::Point]) -> Vec<(f64, f64)> {
    v.iter().map(|p| (p.x, p.y)).collect()
}

fn click_widget(h: &mut Headless, id: &str, count: u64) {
    let r = h.request("ui.clickWidget", json!({"id": id, "count": count}), T);
    assert_eq!(r["ok"], true, "{id}: {r}");
    h.step();
}

/// A finger can't land the precise double-click that deletes a curve point, so the curve's menu
/// (right-click, a long-press on iPad) offers Delete Point for the inner point under the press,
/// and leaves it out for an end point or empty space.
#[test]
fn the_curve_menu_deletes_the_point_under_the_press() {
    let mut h = curve_open("master");
    exec(&mut h, "develop.curve", json!({"channel": "master", "points": [[0.0, 0.0], [0.5, 0.5], [1.0, 1.0]]}));
    h.step();
    let has = |h: &Headless, id: &str| h.app.widgets.iter().any(|(w, _)| w == id);
    // the middle point
    let r = h.request("ui.clickWidget", json!({"id": "curve", "fx": 0.5, "fy": 0.5, "button": "right"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.step();
    click_widget(&mut h, "curveMenu:deletePoint", 1);
    h.step();
    assert_eq!(pts(&develop(&h).curve.master), vec![(0.0, 0.0), (1.0, 1.0)], "the middle point is gone");
    // empty space: the menu has only the reset items
    let r = h.request("ui.clickWidget", json!({"id": "curve", "fx": 0.3, "fy": 0.2, "button": "right"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.step();
    assert!(has(&h, "curveMenu:resetAll"), "the menu opens");
    assert!(!has(&h, "curveMenu:deletePoint"), "no point under the press");
}

/// RawPad (iPad): a long-press on a slider's name (a right-click here) opens a menu whose
/// "Reset Exposure" resets that slider (DECISIONS.md 61). Lives here so CI's `tests_curve` step runs it.
#[test]
fn a_sliders_name_menu_resets_it() {
    let mut h = curve_open("master");
    exec(&mut h, "develop.set", json!({"control": "light.exposure", "value": 1.25}));
    h.step();
    assert!((develop(&h).light.exposure - 1.25).abs() < 1e-9, "exposure set");
    let r = h.request("ui.clickWidget", json!({"id": "sliderLabel:light.exposure", "button": "right"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.step();
    click_widget(&mut h, "sliderMenu:reset:light.exposure", 1);
    h.step();
    assert!(develop(&h).light.exposure.abs() < 1e-9, "exposure back to 0");
}

/// iPad: *Select Several*'s strip in the grid has Sync Settings, which copies the active photo's
/// settings to the other selected photos and says how many (DECISIONS.md 60). Run 54's tour
/// screenshot showed no toast; this checks the button does the work, whatever the screenshot's timing.
#[test]
fn the_select_several_strips_sync_copies_settings_to_the_other_photos() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "photoGrid"}), T);
    assert_eq!(r["ok"], true, "{r}");
    exec(&mut h, "library.select", json!({"ids": [1]}));
    exec(&mut h, "develop.set", json!({"control": "light.exposure", "value": 0.75}));
    exec(&mut h, "library.select", json!({"ids": [1, 2], "active": 1}));
    // the photo's long-press menu › Select Several turns this on
    h.view.ctx.data_mut(|d| d.insert_temp(egui::Id::unique(crate::panels::grid::SELECT_SEVERAL), true));
    h.step();
    h.step();
    let other = |h: &Headless| h.app.session.develop_of(lightcraft_catalog::PhotoId(2)).map(|d| d.light.exposure).unwrap_or_default();
    assert!(other(&h).abs() < 1e-9, "photo 2 starts at 0");
    click_widget(&mut h, "button:selectSeveralSync", 1);
    h.step();
    assert!((other(&h) - 0.75).abs() < 1e-9, "photo 2 follows photo 1: {}", other(&h));
    let toast = h.app.ui.toast.clone().map(|t| t.0).unwrap_or_default();
    assert!(toast.starts_with("Sync Settings"), "the strip says what happened: {toast:?}");
}

/// iPad: a preset's long-press menu shows the preset's look in the loupe while it is open (there is
/// no hover to rest on a preset) and its first item, Apply, applies it (DECISIONS.md 62).
#[test]
fn a_presets_long_press_menu_previews_and_applies_it() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.app.run("panel.presets", json!({})).expect("open the presets column");
    h.step();
    h.step();
    let row = h.app.widgets.iter().map(|(id, _)| id.clone()).find(|id| id.starts_with("preset:")).expect("a preset row on screen");
    let pid = row.trim_start_matches("preset:").to_string();
    let undo = h.app.session.undo.len();
    let r = h.request("ui.clickWidget", json!({"id": row, "button": "right"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    // the finger is gone (no hover left), the menu stays open
    h.app.synthetic.push(egui::Event::PointerMoved(egui::pos2(5.0, 5.0)));
    h.step();
    h.step();
    let preview = h.app.hover_preview.as_ref().map(|p| p.label.clone()).unwrap_or_default();
    assert!(preview.starts_with("Preset"), "the loupe previews the preset while its menu is open: {preview:?}");
    assert_eq!(h.app.session.undo.len(), undo, "previewing changes nothing");
    click_widget(&mut h, &format!("presetMenu:apply:{pid}"), 1);
    h.step();
    let toast = h.app.ui.toast.clone().map(|t| t.0).unwrap_or_default();
    assert!(toast.starts_with("Preset"), "Apply applied it: {toast:?}");
}

/// Opens the long-press (right-click) menu of the first on-screen widget whose id starts with
/// `prefix`, then lifts the finger off it (no hover left) and returns the widget's id.
fn long_press_first(h: &mut Headless, prefix: &str) -> String {
    let row = h.app.widgets.iter().map(|(id, _)| id.clone()).find(|id| id.starts_with(prefix)).unwrap_or_else(|| panic!("a {prefix} row on screen"));
    let r = h.request("ui.clickWidget", json!({"id": row, "button": "right"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.app.synthetic.push(egui::Event::PointerMoved(egui::pos2(5.0, 5.0)));
    h.step();
    h.step();
    row
}

/// iPad: a version's (snapshot's) long-press menu shows it in the loupe while it is open, and its
/// first item restores it (DECISIONS.md 63).
#[test]
fn a_versions_long_press_menu_previews_and_restores_it() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    exec(&mut h, "develop.set", json!({"control": "light.exposure", "value": 0.5}));
    exec(&mut h, "version.create", json!({"name": "Half"}));
    exec(&mut h, "develop.set", json!({"control": "light.exposure", "value": 1.0}));
    h.app.ui.right = crate::state::RightPanel::Versions;
    h.step();
    h.step();
    let row = long_press_first(&mut h, "version:");
    let i = row.trim_start_matches("version:").to_string();
    let preview = h.app.hover_preview.clone().map(|p| (p.label, p.settings.light.exposure));
    let (label, exposure) = preview.unwrap_or_default();
    assert!(label.starts_with("Version"), "the loupe previews the version while its menu is open: {label:?}");
    assert!((exposure - 0.5).abs() < 1e-9, "with the version's settings: {exposure}");
    assert!((develop(&h).light.exposure - 1.0).abs() < 1e-9, "previewing changes nothing");
    click_widget(&mut h, &format!("versionMenu:restore:{i}"), 1);
    h.step();
    assert!((develop(&h).light.exposure - 0.5).abs() < 1e-9, "Restore restored it: {}", develop(&h).light.exposure);
}

/// iPad: a History step's long-press menu shows the photo as it was at that step, and its first
/// item goes back to it (DECISIONS.md 63).
#[test]
fn a_history_steps_long_press_menu_previews_and_goes_back_to_it() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    exec(&mut h, "develop.set", json!({"control": "light.exposure", "value": 0.5}));
    // another control in between, so the two exposure steps can't merge into one
    exec(&mut h, "develop.set", json!({"control": "light.contrast", "value": 10.0}));
    exec(&mut h, "develop.set", json!({"control": "light.exposure", "value": 1.0}));
    h.app.ui.right = crate::state::RightPanel::Activity;
    h.step();
    h.step();
    let id = h.app.session.active().expect("active photo");
    let steps: Vec<f64> = h.app.session.catalog.photo(id).map(|p| p.history.iter().map(|s| s.settings.light.exposure).collect()).unwrap_or_default();
    let i = steps.iter().position(|e| (e - 0.5).abs() < 1e-9).unwrap_or_else(|| panic!("a step at exposure 0.5: {steps:?}"));
    let row = format!("history:{i}");
    let r = h.request("ui.clickWidget", json!({"id": row, "button": "right"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.app.synthetic.push(egui::Event::PointerMoved(egui::pos2(5.0, 5.0)));
    h.step();
    h.step();
    let exposure = h.app.hover_preview.as_ref().map(|p| p.settings.light.exposure);
    assert_eq!(exposure, Some(0.5), "the loupe previews the step while its menu is open");
    assert!((develop(&h).light.exposure - 1.0).abs() < 1e-9, "previewing changes nothing");
    click_widget(&mut h, &format!("historyMenu:restore:{i}"), 1);
    h.step();
    assert!((develop(&h).light.exposure - 0.5).abs() < 1e-9, "Restore went back to the step: {}", develop(&h).light.exposure);
}

/// iPad: with *Select Several* on, the loupe shows a pill above the filmstrip saying how many photos
/// are picked, and its Done turns Select Several off (DECISIONS.md 64). Lives here so CI runs it.
#[test]
fn the_loupe_shows_select_several_above_the_filmstrip_and_done_ends_it() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    exec(&mut h, "library.select", json!({"ids": [1, 2], "active": 1}));
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.step();
    let has = |h: &Headless, id: &str| h.app.widgets.iter().any(|(w, _)| w == id);
    assert!(has(&h, "filmstrip"), "the loupe has its filmstrip");
    assert!(!has(&h, "filmSelectSeveral"), "no pill while Select Several is off");
    h.view.ctx.data_mut(|d| d.insert_temp(egui::Id::unique(crate::panels::grid::SELECT_SEVERAL), true));
    h.step();
    h.step();
    assert!(has(&h, "filmSelectSeveral"), "the pill shows while Select Several is on");
    click_widget(&mut h, "button:filmSelectSeveralDone", 1);
    h.step();
    h.step();
    assert!(!crate::panels::grid::touch_multi(&h.view.ctx), "Done turned Select Several off");
    assert!(!has(&h, "filmSelectSeveral"), "and the pill is gone");
}

/// iPad: a brush or Remove stroke starts where the finger first touched, not where egui noticed the
/// drag (past its threshold): the first few millimetres were lost (editing audit, item 15). Lives
/// here so CI runs it.
#[test]
fn a_brush_or_spot_stroke_starts_where_the_finger_touched() {
    let open = |panel: &str| {
        let services = Services { png: None, ..Default::default() };
        let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
        let mut h = Headless::new(app, [1200.0, 800.0], 1.0);
        let r = h.request("ui.set", json!({"view": "detail"}), T);
        assert_eq!(r["ok"], true, "{r}");
        let r = h.request("engine.execute", json!({"command": panel}), T);
        assert_eq!(r["ok"], true, "{r}");
        h.step();
        h
    };
    let stroke = |h: &mut Headless| {
        let events = json!([
            {"kind": "down", "x": 0.15, "y": 0.4},
            {"kind": "drag", "x": 0.3, "y": 0.4},
            {"kind": "drag", "x": 0.5, "y": 0.4},
            {"kind": "up", "x": 0.5, "y": 0.4}
        ]);
        let r = h.request("ui.pointer", json!({"events": events}), T);
        assert_eq!(r["ok"], true, "{r}");
    };
    let min_x = |v: &[lightcraft_geom::Point]| v.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    // brush
    let mut h = open("panel.masking");
    exec(&mut h, "tool.brush", json!({"new": true}));
    stroke(&mut h);
    let d = develop(&h);
    let lightcraft_develop::MaskShape::Brush { strokes } = &d.masks[0].components[0].shape else { panic!("a brush mask") };
    assert!((min_x(&strokes[0].points) - 0.15).abs() < 0.02, "the brush stroke starts at the press: {:?}", strokes[0].points);
    // Remove (spot) stroke
    let mut h = open("panel.remove");
    stroke(&mut h);
    let spots = develop(&h).spots;
    assert_eq!(spots.len(), 1, "one spot");
    assert!((min_x(&spots[0].points) - 0.15).abs() < 0.02, "the spot stroke starts at the press: {:?}", spots[0].points);
}

/// iPad: the bottom bar's rating pill ends in a colour-label button whose menu sets and clears the
/// active photo's label (editing audit, item 30: labels were out of reach by touch; DECISIONS 80).
/// Lives here so CI runs it.
#[test]
fn the_bottom_bars_label_button_sets_and_clears_the_colour_label() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.step();
    let id = h.app.session.active().expect("active photo");
    let label = |h: &Headless| h.app.session.catalog.photo(id).and_then(|p| p.label);
    let rect = |h: &Headless, id: &str| h.app.widgets.iter().find(|(w, _)| w == id).map(|(_, r)| *r);
    let button = rect(&h, "icon:labelMenu").expect("the pill has the label button");
    let last_flag = rect(&h, "flag:reject").expect("the pill has its flags");
    assert!(button.left() >= last_flag.right(), "the label button comes after the flags: {button:?} {last_flag:?}");
    click_widget(&mut h, "icon:labelMenu", 1);
    h.step();
    click_widget(&mut h, "bottomLabel:purple", 1);
    h.step();
    assert_eq!(label(&h), Some(lightcraft_catalog::ColorLabel::Purple), "Purple set from the bottom bar");
    click_widget(&mut h, "icon:labelMenu", 1);
    h.step();
    click_widget(&mut h, "bottomLabel:none", 1);
    h.step();
    assert_eq!(label(&h), None, "None clears it");
}

/// iPad: when the bottom bar is too narrow for the full rating pill (portrait with the side
/// toolbars), a compact pill keeps rating, flag and colour label in reach: a rating button whose
/// menu holds the stars and flags, then the label button (DECISIONS 81). Lives here so CI runs it.
#[test]
fn a_narrow_bottom_bar_shows_the_compact_rating_pill() {
    let services = Services { png: None, ..Default::default() };
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), services);
    let mut h = Headless::new(app, [1200.0, 1400.0], 1.0);
    let r = h.request("ui.set", json!({"view": "detail", "right": "none"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.step();
    h.step();
    let has = |h: &Headless, id: &str| h.app.widgets.iter().any(|(w, _)| w == id);
    assert!(has(&h, "flag:reject") && !has(&h, "icon:ratingMenu"), "a wide bar has the full pill");
    // narrow the window until the full pill gives way
    let mut found = false;
    for w in (500..=1200).rev().step_by(10) {
        let r = h.request("ui.resize", json!({"width": w, "height": 1400}), T);
        assert_eq!(r["ok"], true, "{r}");
        h.step();
        h.step();
        assert!(!(has(&h, "flag:reject") && has(&h, "icon:ratingMenu")), "never both pills at {w}");
        if has(&h, "icon:ratingMenu") {
            found = true;
            break;
        }
    }
    assert!(found, "some narrow width shows the compact pill");
    assert!(has(&h, "icon:labelMenu"), "the compact pill keeps the label button");
    let id = h.app.session.active().expect("active photo");
    let photo = |h: &Headless| h.app.session.catalog.photo(id).cloned().expect("photo");
    click_widget(&mut h, "icon:ratingMenu", 1);
    h.step();
    click_widget(&mut h, "star:bottom:4", 1);
    h.step();
    assert_eq!(photo(&h).rating, 4, "four stars from the compact pill's menu");
    if !has(&h, "flag:pick") {
        click_widget(&mut h, "icon:ratingMenu", 1);
        h.step();
    }
    click_widget(&mut h, "flag:pick", 1);
    h.step();
    assert_eq!(photo(&h).flag, lightcraft_catalog::Flag::Pick, "picked from the compact pill's menu");
    if has(&h, "flag:pick") {
        // the menu stayed open: close it before the label button
        let _ = h.request("ui.key", json!({"key": "esc"}), T);
        h.step();
    }
    click_widget(&mut h, "icon:labelMenu", 1);
    h.step();
    click_widget(&mut h, "bottomLabel:green", 1);
    h.step();
    assert_eq!(photo(&h).label, Some(lightcraft_catalog::ColorLabel::Green), "Green from the compact pill");
}
