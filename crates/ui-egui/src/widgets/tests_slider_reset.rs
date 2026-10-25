//! Real egui pointer frames on a disabled slider: its name doesn't reset it (issue #1436).
use super::{SliderOut, slider, take_registry};
use crate::headless::HeadlessView;
use egui::{Event, Modifiers, PointerButton, Rect, vec2};
use lightcraft_develop::controls;

fn frame(view: &mut HeadlessView, time: f64, enabled: bool, events: Vec<Event>) -> (SliderOut, Rect) {
    let spec = controls::find("light.exposure").unwrap();
    let mut out = SliderOut::default();
    view.run(HeadlessView::raw_input(vec2(330.0, 100.0), 1.0, time, events), |ui| {
        out = slider(ui, spec, 1.25, enabled, None);
    });
    let name = take_registry(&view.ctx).into_iter().find(|(id, _)| id == "sliderLabel:light.exposure").unwrap().1;
    (out, name)
}

#[test]
fn a_disabled_sliders_name_ignores_a_double_click() {
    for enabled in [true, false] {
        let mut view = HeadlessView::new();
        let mut time = 0.0;
        let (_, name) = frame(&mut view, time, enabled, vec![]);
        let pos = name.left_center() + vec2(8.0, 0.0);
        let mut reset = false;
        for pressed in [None, Some(true), Some(false), Some(true), Some(false), None] {
            time += 0.02;
            let events = match pressed {
                None => vec![Event::PointerMoved(pos)],
                Some(pressed) => vec![Event::PointerButton { pos, button: PointerButton::Primary, pressed, modifiers: Modifiers::NONE }],
            };
            let (out, _) = frame(&mut view, time, enabled, events);
            reset |= out.reset || out.value.is_some();
        }
        assert_eq!(reset, enabled, "enabled: {enabled}");
    }
}

/// RawPad (iPad): a long-press on the name (a secondary click) opens a menu whose
/// "Reset Exposure" resets the slider (DECISIONS.md 61).
#[test]
fn the_names_menu_resets_the_slider() {
    let spec = controls::find("light.exposure").unwrap();
    let mut view = HeadlessView::new();
    let mut time = 0.0;
    let step = |view: &mut HeadlessView, time: f64, events: Vec<Event>| {
        let mut out = SliderOut::default();
        view.run(HeadlessView::raw_input(vec2(330.0, 300.0), 1.0, time, events), |ui| {
            out = slider(ui, spec, 1.25, true, None);
        });
        (out, take_registry(&view.ctx))
    };
    let (_, reg) = step(&mut view, time, vec![]);
    let name = reg.iter().find(|(id, _)| id == "sliderLabel:light.exposure").unwrap().1;
    let pos = name.left_center() + vec2(8.0, 0.0);
    let mut item = None;
    for pressed in [None, Some(true), Some(false), None, None] {
        time += 0.02;
        let events = match pressed {
            None => vec![Event::PointerMoved(pos)],
            Some(pressed) => vec![Event::PointerButton { pos, button: PointerButton::Secondary, pressed, modifiers: Modifiers::NONE }],
        };
        let (out, reg) = step(&mut view, time, events);
        assert!(!out.reset && out.value.is_none(), "opening the menu changes nothing");
        item = item.or_else(|| reg.iter().find(|(id, _)| id == "sliderMenu:reset:light.exposure").map(|(_, r)| *r));
    }
    let item = item.expect("the name's menu shows Reset Exposure");
    let at = item.center();
    let mut reset = false;
    for pressed in [None, Some(true), Some(false), None] {
        time += 0.02;
        let events = match pressed {
            None => vec![Event::PointerMoved(at)],
            Some(pressed) => vec![Event::PointerButton { pos: at, button: PointerButton::Primary, pressed, modifiers: Modifiers::NONE }],
        };
        let (out, _) = step(&mut view, time, events);
        if out.reset {
            assert_eq!(out.value, Some(spec.default));
            reset = true;
        }
    }
    assert!(reset, "Reset Exposure resets the slider");
}
