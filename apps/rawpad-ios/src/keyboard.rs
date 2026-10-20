//! Keyboard avoidance: the on-screen keyboard covers the bottom of the screen and egui doesn't
//! know. While a focused text field would sit under the keyboard, the host shrinks egui's screen
//! rect by the keyboard's height, so the panels lay out above it (DECISIONS.md 45).
//!
//! The decision is made once per focused field and kept until the field loses focus or the
//! keyboard goes away: after the UI shrinks, the field moves up and no longer looks covered, and
//! re-deciding then would make the layout jump back and forth. Plain egui types, so the tests run
//! on the CI's macOS host too.

/// Points kept free between a field's bottom edge and the keyboard's top edge.
pub const MARGIN: f32 = 12.0;

#[derive(Debug, Default)]
pub struct Avoid {
    /// The focused field the UI was shrunk for.
    shrunk_for: Option<egui::Id>,
}

impl Avoid {
    /// The bottom inset (egui points) for this frame.
    ///
    /// - `cover`: how far the docked on-screen keyboard reaches up from the bottom of the full
    ///   screen rect (0 when hidden, floating or replaced by a hardware keyboard's thin bar).
    /// - `focused`: the egui text field with keyboard focus (last frame), if any.
    /// - `field_bottom`: that field's bottom edge (last frame, full-screen coordinates), if known.
    /// - `screen_bottom`: the bottom of the full screen rect.
    pub fn inset(&mut self, cover: f32, focused: Option<egui::Id>, field_bottom: Option<f32>, screen_bottom: f32) -> f32 {
        let cover = if cover.is_finite() { cover.max(0.0) } else { 0.0 };
        let Some(id) = focused.filter(|_| cover > 0.0) else {
            self.shrunk_for = None;
            return 0.0;
        };
        if self.shrunk_for != Some(id) {
            self.shrunk_for = None;
            // unknown position: shrink to be safe (the field may be anywhere)
            let covered = field_bottom.is_none_or(|b| !b.is_finite() || b + MARGIN > screen_bottom - cover);
            if covered {
                self.shrunk_for = Some(id);
            }
        }
        if self.shrunk_for == Some(id) { cover } else { 0.0 }
    }

    /// Whether the UI is currently shrunk for the keyboard.
    pub fn active(&self) -> bool {
        self.shrunk_for.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u64) -> Option<egui::Id> {
        Some(egui::Id::new(n))
    }

    #[test]
    fn no_keyboard_or_no_focus_means_no_inset() {
        let mut a = Avoid::default();
        assert_eq!(a.inset(0.0, id(1), Some(900.0), 1000.0), 0.0);
        assert_eq!(a.inset(400.0, None, None, 1000.0), 0.0);
        assert!(!a.active());
    }

    #[test]
    fn field_above_the_keyboard_keeps_the_layout() {
        let mut a = Avoid::default();
        // search box near the top
        assert_eq!(a.inset(400.0, id(1), Some(60.0), 1000.0), 0.0);
        assert!(!a.active());
    }

    #[test]
    fn covered_field_shrinks_and_stays_shrunk_after_it_moves_up() {
        let mut a = Avoid::default();
        assert_eq!(a.inset(400.0, id(2), Some(900.0), 1000.0), 400.0);
        // next frame the field sits higher, above the keyboard: still shrunk (no jumping)
        assert_eq!(a.inset(400.0, id(2), Some(560.0), 1000.0), 400.0);
        assert!(a.active());
        // keyboard gone: back to full height
        assert_eq!(a.inset(0.0, id(2), Some(560.0), 1000.0), 0.0);
        assert!(!a.active());
    }

    #[test]
    fn keyboard_growing_during_its_animation_is_caught() {
        let mut a = Avoid::default();
        // first frames: the keyboard is still sliding in and doesn't reach the field yet
        assert_eq!(a.inset(50.0, id(3), Some(800.0), 1000.0), 0.0);
        assert_eq!(a.inset(250.0, id(3), Some(800.0), 1000.0), 250.0);
        assert_eq!(a.inset(400.0, id(3), Some(500.0), 1000.0), 400.0);
    }

    #[test]
    fn another_field_is_decided_afresh() {
        let mut a = Avoid::default();
        assert_eq!(a.inset(400.0, id(4), Some(900.0), 1000.0), 400.0);
        assert_eq!(a.inset(400.0, id(5), Some(60.0), 1000.0), 0.0);
    }

    #[test]
    fn unknown_position_or_bad_numbers_are_safe() {
        let mut a = Avoid::default();
        assert_eq!(a.inset(400.0, id(6), None, 1000.0), 400.0);
        let mut b = Avoid::default();
        assert_eq!(b.inset(f32::NAN, id(6), Some(900.0), 1000.0), 0.0);
        assert_eq!(b.inset(-5.0, id(6), Some(900.0), 1000.0), 0.0);
        assert_eq!(b.inset(400.0, id(6), Some(f32::INFINITY), 1000.0), 400.0);
    }
}
