//! Hardware-keyboard shortcuts: ⌘ on an iPad keyboard is egui's `command`.
//!
//! egui-winit only treats the logo key (⌘) as `command` on macOS; on other targets, iOS
//! included, `command` follows Ctrl. On the iPad, as on the Mac, LightCraft's shortcuts
//! (⌘Z, ⌘C, ⌘E…) must come from ⌘, so the host tracks ⌘ from winit's `ModifiersChanged` and
//! rewrites the modifiers of this frame's events and of the input as a whole.

use egui::{Event, Modifiers};

/// Mac-style modifiers from what the keyboard holds: `command` and `mac_cmd` are ⌘ alone.
pub fn mac_like(mut m: Modifiers, logo: bool) -> Modifiers {
    m.mac_cmd = logo;
    m.command = logo;
    m
}

/// Rewrite every event's modifiers and the input's (see the module docs).
pub fn apply(events: &mut [Event], modifiers: &mut Modifiers, logo: bool) {
    *modifiers = mac_like(*modifiers, logo);
    for event in events.iter_mut() {
        match event {
            Event::Key { modifiers, .. } | Event::PointerButton { modifiers, .. } | Event::MouseWheel { modifiers, .. } => {
                *modifiers = mac_like(*modifiers, logo);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Key;

    fn key(modifiers: Modifiers) -> Event {
        Event::Key { key: Key::Z, physical_key: Some(Key::Z), pressed: true, repeat: false, modifiers }
    }

    #[test]
    fn logo_key_is_command() {
        let mut ev = vec![key(Modifiers::NONE)];
        let mut m = Modifiers::NONE;
        apply(&mut ev, &mut m, true);
        assert!(m.command && m.mac_cmd);
        assert!(matches!(&ev[0], Event::Key { modifiers, .. } if modifiers.command && modifiers.mac_cmd && !modifiers.ctrl));
    }

    #[test]
    fn ctrl_alone_is_not_command() {
        // what egui-winit reports on iOS for Ctrl: ctrl and command both set
        let ctrl = Modifiers { ctrl: true, command: true, ..Modifiers::NONE };
        let mut ev = vec![key(ctrl)];
        let mut m = ctrl;
        apply(&mut ev, &mut m, false);
        assert!(m.ctrl && !m.command);
        assert!(matches!(&ev[0], Event::Key { modifiers, .. } if modifiers.ctrl && !modifiers.command));
    }
}
