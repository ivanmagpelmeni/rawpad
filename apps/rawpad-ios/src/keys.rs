//! Hardware-keyboard shortcuts: ⌘ on an iPad keyboard is egui's `command`.
//!
//! egui-winit only treats the logo key (⌘) as `command` on macOS; on other targets, iOS
//! included, `command` follows Ctrl. On the iPad, as on the Mac, LightCraft's shortcuts
//! (⌘Z, ⌘C, ⌘E…) must come from ⌘, so the host tracks ⌘ from winit's `ModifiersChanged` and
//! rewrites the modifiers of this frame's events (egui 0.36 reads them from the events).
//!
//! winit 0.30 may pass no hardware keys on iOS at all (CI run 30), so `native::KeyView` also
//! reads UIKit's presses itself; [`uikit_events`] turns them into egui events.

use egui::{Event, Key, Modifiers};

/// Mac-style modifiers from what the keyboard holds: `command` and `mac_cmd` are ⌘ alone.
pub fn mac_like(mut m: Modifiers, logo: bool) -> Modifiers {
    m.mac_cmd = logo;
    m.command = logo;
    m
}

/// Rewrite every event's modifiers (see the module docs).
pub fn apply(events: &mut [Event], logo: bool) {
    for event in events.iter_mut() {
        match event {
            Event::Key { modifiers, .. } | Event::PointerButton { modifiers, .. } | Event::MouseWheel { modifiers, .. } => {
                *modifiers = mac_like(*modifiers, logo);
            }
            _ => {}
        }
    }
}

/// `UIKeyModifierFlags` bits (UIKit's `UICommand.h`).
const UI_SHIFT: usize = 1 << 17;
const UI_CONTROL: usize = 1 << 18;
const UI_ALTERNATE: usize = 1 << 19;
const UI_COMMAND: usize = 1 << 20;

/// egui modifiers from a UIKit key's `modifierFlags`, Mac-style (⌘ is `command`).
pub fn uikit_modifiers(flags: usize) -> Modifiers {
    let cmd = flags & UI_COMMAND != 0;
    Modifiers { alt: flags & UI_ALTERNATE != 0, ctrl: flags & UI_CONTROL != 0, shift: flags & UI_SHIFT != 0, mac_cmd: cmd, command: cmd }
}

/// The modifier flags to remember after a press or release. UIKit reports the releasing ⌘ (or
/// ⇧, ⌥, Ctrl) key with its own flag still set (CI run 33: `key 0xE3 flags 0x100000 up`), so the
/// released modifier's bit is cleared here; otherwise ⌘ would stick and every tap would be a ⌘-tap.
pub fn uikit_flags_after(code: isize, flags: usize, pressed: bool) -> usize {
    let bit = match code {
        0xE0 | 0xE4 => UI_CONTROL,
        0xE1 | 0xE5 => UI_SHIFT,
        0xE2 | 0xE6 => UI_ALTERNATE,
        0xE3 | 0xE7 => UI_COMMAND,
        _ => 0,
    };
    if pressed { flags | bit } else { flags & !bit }
}

/// The egui key for a USB HID keyboard usage (`UIKeyboardHIDUsage`, what UIKit's `UIKey.keyCode`
/// holds). Keys egui has no name for give `None`.
pub fn hid_key(code: isize) -> Option<Key> {
    use Key::*;
    const LETTERS: [Key; 26] = [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z];
    const DIGITS: [Key; 10] = [Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0];
    const FKEYS: [Key; 12] = [F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12];
    let at = |table: &[Key], first: isize| usize::try_from(code - first).ok().and_then(|i| table.get(i).copied());
    Some(match code {
        0x04..=0x1D => return at(&LETTERS, 0x04),
        0x1E..=0x27 => return at(&DIGITS, 0x1E),
        // the numeric keypad's digits (KP1…KP9, KP0: the same order); osascript's "5" is KP5
        0x59..=0x62 => return at(&DIGITS, 0x59),
        0x3A..=0x45 => return at(&FKEYS, 0x3A),
        0x28 | 0x58 => Enter,
        0x29 => Escape,
        0x2A => Backspace,
        0x2B => Tab,
        0x2C => Space,
        0x2D | 0x56 => Minus,
        0x2E => Equals,
        0x57 => Plus,
        0x2F => OpenBracket,
        0x30 => CloseBracket,
        0x31 => Backslash,
        0x33 => Semicolon,
        0x34 => Quote,
        0x35 => Backtick,
        0x36 => Comma,
        0x37 | 0x63 => Period,
        0x38 | 0x54 => Slash,
        0x49 => Insert,
        0x4A => Home,
        0x4B => PageUp,
        0x4C => Delete,
        0x4D => End,
        0x4E => PageDown,
        0x4F => ArrowRight,
        0x50 => ArrowLeft,
        0x51 => ArrowDown,
        0x52 => ArrowUp,
        _ => return None,
    })
}

/// The egui events for one UIKit key press or release (`pressesBegan` / `pressesEnded`): a key
/// event, plus the typed text on a press without ⌘ or Ctrl (control characters and UIKit's
/// private-use function-key characters are not text).
pub fn uikit_events(code: isize, flags: usize, characters: &str, pressed: bool) -> Vec<Event> {
    let modifiers = uikit_modifiers(flags);
    let mut out = Vec::with_capacity(2);
    if let Some(key) = hid_key(code) {
        out.push(Event::Key { key, physical_key: Some(key), pressed, repeat: false, modifiers });
    }
    let text = pressed && !modifiers.command && !modifiers.ctrl && !characters.is_empty();
    if text && characters.chars().all(|c| !c.is_control() && !('\u{E000}'..='\u{F8FF}').contains(&c)) {
        out.push(Event::Text(characters.to_string()));
    }
    out
}

/// A tap on a key: pressed and released, no modifiers.
fn tap(key: Key) -> [Event; 2] {
    let ev = |pressed| Event::Key { key, physical_key: Some(key), pressed, repeat: false, modifiers: Modifiers::NONE };
    [ev(true), ev(false)]
}

/// The egui events for text from the on-screen keyboard (`UIKeyInput.insertText:`): Return is the
/// Enter key, Tab the Tab key, anything else typed text.
pub fn soft_text_events(text: &str) -> Vec<Event> {
    let mut out = Vec::new();
    let mut run = String::new();
    for c in text.chars() {
        let key = match c {
            '\n' | '\r' => Some(Key::Enter),
            '\t' => Some(Key::Tab),
            c if c.is_control() => continue,
            _ => None,
        };
        match key {
            Some(key) => {
                if !run.is_empty() {
                    out.push(Event::Text(std::mem::take(&mut run)));
                }
                out.extend(tap(key));
            }
            None => run.push(c),
        }
    }
    if !run.is_empty() {
        out.push(Event::Text(run));
    }
    out
}

/// The on-screen keyboard's delete key (`UIKeyInput.deleteBackward`).
pub fn soft_backspace() -> Vec<Event> {
    tap(Key::Backspace).to_vec()
}

/// How long after a tap a text field that takes the focus may still bring up the on-screen
/// keyboard. A tap's press and release are a few frames apart, and egui gives the field its focus
/// on one of them and reports it (`PlatformOutput::ime`) a frame later.
pub const SOFT_KEYBOARD_TAP_WINDOW: std::time::Duration = std::time::Duration::from_millis(800);

/// How far outside a text box a tap still counts as a tap on it (points).
const SOFT_KEYBOARD_TAP_SLOP: f32 = 8.0;

/// Where the last tap or press of this frame's events was (a finger, the Pencil, or the CI's
/// pointer), if any: the only way a text box may bring up the on-screen keyboard
/// (DECISIONS.md 45).
pub fn tap_pos(events: &[Event]) -> Option<egui::Pos2> {
    events.iter().rev().find_map(|e| match e {
        Event::PointerButton { pos, button: egui::PointerButton::Primary, .. } => Some(*pos),
        Event::Touch { pos, phase: egui::TouchPhase::Start | egui::TouchPhase::End, .. } => Some(*pos),
        _ => None,
    })
}

/// Whether the on-screen keyboard should be up (DECISIONS.md 45). `field`: the rectangle of the
/// egui text box being edited, if any (`PlatformOutput::ime`, last frame: only a focused, editable
/// `TextEdit` sets it; a focused slider, the photo or a mask never does). `tap`: how long ago the
/// last tap was, and where. The keyboard comes up only when a text box has the focus right after
/// a tap on that box (or on the slider value that opens one in its place), stays up while a text
/// box keeps the focus, and goes away when none has it. A box focused by code after a tap
/// elsewhere (a menu's search field, a dialog's name field) leaves it down until it is tapped; a
/// hardware keyboard types into it anyway.
pub fn soft_keyboard(was_up: bool, field: Option<egui::Rect>, tap: Option<(std::time::Duration, egui::Pos2)>) -> bool {
    field.is_some_and(|rect| {
        was_up || tap.is_some_and(|(ago, at)| ago <= SOFT_KEYBOARD_TAP_WINDOW && rect.expand(SOFT_KEYBOARD_TAP_SLOP).contains(at))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soft_keyboard_text_and_return() {
        let ev = soft_text_events("ab\nc");
        assert!(matches!(&ev[0], Event::Text(t) if t == "ab"));
        assert!(matches!(&ev[1], Event::Key { key: Key::Enter, pressed: true, .. }));
        assert!(matches!(&ev[2], Event::Key { key: Key::Enter, pressed: false, .. }));
        assert!(matches!(&ev[3], Event::Text(t) if t == "c"));
        assert_eq!(ev.len(), 4);
        assert!(soft_text_events("\u{7}").is_empty(), "control characters are dropped");
        assert!(matches!(&soft_text_events("ü")[0], Event::Text(t) if t == "ü"));
        let bs = soft_backspace();
        assert!(matches!(&bs[0], Event::Key { key: Key::Backspace, pressed: true, .. }));
        assert_eq!(bs.len(), 2);
    }

    #[test]
    fn soft_keyboard_only_for_a_tapped_text_field() {
        use std::time::Duration;
        let field = egui::Rect::from_min_size(egui::pos2(300.0, 20.0), egui::vec2(220.0, 28.0));
        let on_field = egui::pos2(400.0, 34.0);
        let elsewhere = egui::pos2(400.0, 600.0);
        let tap = |ms, at| Some((Duration::from_millis(ms), at));
        // a slider, the photo, a mask: focused or dragged, but no text box → never
        assert!(!soft_keyboard(false, None, tap(0, elsewhere)));
        assert!(!soft_keyboard(true, None, tap(0, on_field)), "the keyboard goes when the box loses the focus");
        // a tap on the search box (or a slider's value) → up, and it stays up while typing
        assert!(soft_keyboard(false, Some(field), tap(50, on_field)));
        assert!(soft_keyboard(false, Some(field), tap(50, egui::pos2(296.0, 34.0))), "a near miss counts");
        assert!(soft_keyboard(true, Some(field), None));
        assert!(soft_keyboard(true, Some(field), tap(60_000, elsewhere)));
        // a box focused by code: after a tap elsewhere (a menu's search), or long after any tap
        assert!(!soft_keyboard(false, Some(field), tap(50, elsewhere)));
        assert!(!soft_keyboard(false, Some(field), None));
        assert!(!soft_keyboard(false, Some(field), tap(5_000, on_field)));
    }

    #[test]
    fn taps_are_presses_and_touch_starts_or_ends() {
        let (a, b) = (egui::pos2(10.0, 10.0), egui::pos2(50.0, 60.0));
        let press = |pos| Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE };
        assert_eq!(tap_pos(&[Event::PointerMoved(a), press(a), press(b)]), Some(b), "the last tap counts");
        assert_eq!(tap_pos(&[Event::PointerMoved(a)]), None, "a drag's moves are not a tap");
        let touch = |phase| Event::Touch { device_id: egui::TouchDeviceId(1), id: egui::TouchId(1), phase, pos: a, force: None };
        assert_eq!(tap_pos(&[touch(egui::TouchPhase::End)]), Some(a));
        assert_eq!(tap_pos(&[touch(egui::TouchPhase::Move)]), None);
        assert_eq!(tap_pos(&soft_text_events("a")), None, "typing is not a tap");
    }

    fn key(modifiers: Modifiers) -> Event {
        Event::Key { key: Key::Z, physical_key: Some(Key::Z), pressed: true, repeat: false, modifiers }
    }

    #[test]
    fn logo_key_is_command() {
        let mut ev = vec![key(Modifiers::NONE)];
        apply(&mut ev, true);
        assert!(mac_like(Modifiers::NONE, true).command);
        assert!(matches!(&ev[0], Event::Key { modifiers, .. } if modifiers.command && modifiers.mac_cmd && !modifiers.ctrl));
    }

    #[test]
    fn ctrl_alone_is_not_command() {
        // what egui-winit reports on iOS for Ctrl: ctrl and command both set
        let ctrl = Modifiers { ctrl: true, command: true, ..Modifiers::NONE };
        let mut ev = vec![key(ctrl)];
        apply(&mut ev, false);
        assert!(matches!(&ev[0], Event::Key { modifiers, .. } if modifiers.ctrl && !modifiers.command));
    }

    #[test]
    fn uikit_command_z_is_undo_shortcut() {
        // ⌘Z: HID 0x1D, UIKeyModifierCommand
        let ev = uikit_events(0x1D, 1 << 20, "z", true);
        assert_eq!(ev.len(), 1, "no text with ⌘");
        assert!(
            matches!(&ev[0], Event::Key { key: Key::Z, pressed: true, modifiers, .. } if modifiers.command && modifiers.mac_cmd && !modifiers.ctrl)
        );
    }

    #[test]
    fn uikit_plain_letter_types_text() {
        let ev = uikit_events(0x13, 1 << 17, "P", true);
        assert!(matches!(&ev[0], Event::Key { key: Key::P, modifiers, .. } if modifiers.shift && !modifiers.command));
        assert!(matches!(&ev[1], Event::Text(t) if t == "P"));
        // a release types nothing
        assert_eq!(uikit_events(0x13, 0, "p", false).len(), 1);
    }

    #[test]
    fn uikit_hid_table() {
        assert_eq!(hid_key(0x04), Some(Key::A));
        assert_eq!(hid_key(0x27), Some(Key::Num0));
        // keypad 5 (what CI run 35 got for "5", with UIKeyModifierNumericPad) and keypad 0
        assert_eq!(hid_key(0x5D), Some(Key::Num5));
        assert_eq!(hid_key(0x62), Some(Key::Num0));
        assert!(matches!(&uikit_events(0x5D, 1 << 21, "5", true)[0], Event::Key { key: Key::Num5, modifiers, .. } if !modifiers.command));
        assert_eq!(hid_key(0x45), Some(Key::F12));
        assert_eq!(hid_key(0x52), Some(Key::ArrowUp));
        assert_eq!(hid_key(0x29), Some(Key::Escape));
        assert_eq!(hid_key(-5), None);
        assert_eq!(hid_key(0xE3), None, "⌘ itself is a modifier, not a key");
        // arrow keys come with a private-use character: no text
        assert_eq!(uikit_events(0x4F, 0, "\u{F703}", true).len(), 1);
        assert_eq!(uikit_events(0x28, 0, "\r", true).len(), 1);
    }

    #[test]
    fn released_command_does_not_stick() {
        // what UIKit sent in run 33 for ⌘Z
        let mut flags = uikit_flags_after(0xE3, 1 << 20, true);
        assert!(uikit_modifiers(flags).command);
        flags = uikit_flags_after(0x1D, 1 << 20, false);
        assert!(uikit_modifiers(flags).command, "Z released, ⌘ still held");
        flags = uikit_flags_after(0xE3, 1 << 20, false);
        assert!(!uikit_modifiers(flags).command);
        assert!(!uikit_modifiers(uikit_flags_after(0xE5, 1 << 17, false)).shift);
    }
}
