//! Long-press → secondary click, so touch can reach context menus (grid, folders, masks…).
//!
//! egui-winit turns the first finger into a simulated mouse: a `Touch` start, then a primary
//! `PointerButton` press. This filter holds that press back for a moment:
//! - the finger lifts first → the press and release go through together (a tap);
//! - the finger moves more than [`SLOP`] → the press goes through, then the moves (a drag/scroll);
//! - a second finger lands → the press is dropped (pinch and pan stay egui's, and no tool sees a
//!   stray one-finger press; it used to go through);
//! - the finger stays still for [`HOLD`] → a secondary click at that spot instead, and the primary
//!   release that follows is dropped so it can't close the menu that just opened.
//!
//! A quick tap with two fingers is Undo and with three Redo, as in other iPad drawing apps
//! ([`LongPress::take_gesture`], DECISIONS.md 49): every finger lifts within [`MULTI_TAP`] of the
//! first landing and none wandered more than [`SLOP`].
//!
//! Presses that don't come from a touch (a trackpad or mouse) are never held back, and neither
//! are Apple Pencil touches ([`LongPress::mark_pencil`]): a brush stroke starts the moment the tip
//! lands, even when it rests there first.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use egui::{Event, Modifiers, PointerButton, Pos2, TouchPhase};

/// How long a still finger takes to become a secondary click (iOS uses about the same).
pub const HOLD: Duration = Duration::from_millis(500);
/// How far (in points) a finger may wander and still count as still.
pub const SLOP: f32 = 10.0;
/// How quick a two- or three-finger tap must be, from the first finger landing to the last lifting.
pub const MULTI_TAP: Duration = Duration::from_millis(350);

/// A multi-finger tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gesture {
    /// Two fingers: Undo.
    Undo,
    /// Three fingers: Redo.
    Redo,
}

/// The fingers of the current touch sequence (first landing to last lifting), for multi-finger taps.
#[derive(Debug)]
struct Fingers {
    start: Instant,
    /// The most fingers down at once.
    most: usize,
    landed: BTreeMap<u64, Pos2>,
    /// A finger wandered, was cancelled or was the Pencil: not a tap.
    spoiled: bool,
}

#[derive(Debug, Default)]
enum State {
    #[default]
    Idle,
    /// A touch press held back since `start`; `buffered` are the events that came after it.
    Pending { pos: Pos2, modifiers: Modifiers, start: Instant, buffered: Vec<Event> },
    /// A long-press fired: drop the primary release that ends it.
    Swallow,
}

#[derive(Debug, Default)]
pub struct LongPress {
    state: State,
    touches: BTreeSet<u64>,
    /// Touches made with Apple Pencil (marked by the host from winit's events, before egui's).
    pencils: BTreeSet<u64>,
    fingers: Option<Fingers>,
    gesture: Option<Gesture>,
}

impl LongPress {
    /// The touch `id` (winit's, which is also egui's `TouchId`) comes from Apple Pencil: its press
    /// goes through at once instead of waiting to become a long-press.
    pub fn mark_pencil(&mut self, id: u64) {
        self.pencils.insert(id);
    }

    /// The multi-finger tap that ended in the last [`LongPress::filter`], if any.
    pub fn take_gesture(&mut self) -> Option<Gesture> {
        self.gesture.take()
    }

    /// Rewrite this frame's events. Returns how long until the held press becomes a long-press,
    /// so the host can wake up then even without new input.
    pub fn filter(&mut self, events: &mut Vec<Event>, now: Instant) -> Option<Duration> {
        let mut out = Vec::with_capacity(events.len() + 2);
        for event in events.drain(..) {
            self.step(event, now, &mut out);
        }
        let wake = match &mut self.state {
            State::Pending { pos, start, .. } => {
                let held = now.saturating_duration_since(*start);
                if held >= HOLD {
                    let pos = *pos;
                    // the small wobble inside SLOP is dropped: the menu opens where the finger landed
                    self.state = State::Swallow;
                    out.push(Event::PointerMoved(pos));
                    for pressed in [true, false] {
                        out.push(Event::PointerButton { pos, button: PointerButton::Secondary, pressed, modifiers: Modifiers::NONE });
                    }
                    None
                } else {
                    Some(HOLD.saturating_sub(held))
                }
            }
            _ => None,
        };
        *events = out;
        wake
    }

    fn step(&mut self, event: Event, now: Instant, out: &mut Vec<Event>) {
        if let Event::Touch { id, phase, pos, .. } = &event {
            match phase {
                TouchPhase::Start => {
                    self.touches.insert(id.0);
                    let pencil = self.pencils.contains(&id.0);
                    let fingers = self.fingers.get_or_insert_with(|| Fingers { start: now, most: 0, landed: BTreeMap::new(), spoiled: false });
                    fingers.landed.insert(id.0, *pos);
                    fingers.most = fingers.most.max(self.touches.len());
                    fingers.spoiled |= pencil;
                    if self.touches.len() > 1 {
                        // a second finger: a pinch, a pan or a multi-finger tap, never a
                        // long-press or a one-finger stroke: the held press is dropped
                        self.drop_pending();
                    }
                }
                TouchPhase::Move => {
                    if let Some(fingers) = self.fingers.as_mut()
                        && fingers.landed.get(&id.0).is_some_and(|p| p.distance(*pos) > SLOP)
                    {
                        fingers.spoiled = true;
                    }
                }
                TouchPhase::End | TouchPhase::Cancel => {
                    self.touches.remove(&id.0);
                    self.pencils.remove(&id.0);
                    if let Some(fingers) = self.fingers.as_mut() {
                        fingers.spoiled |= *phase == TouchPhase::Cancel || fingers.landed.get(&id.0).is_some_and(|p| p.distance(*pos) > SLOP);
                    }
                    if self.touches.is_empty() {
                        self.pencils.clear();
                        if let Some(fingers) = self.fingers.take()
                            && !fingers.spoiled
                            && now.saturating_duration_since(fingers.start) <= MULTI_TAP
                        {
                            match fingers.most {
                                2 => self.gesture = Some(Gesture::Undo),
                                3 => self.gesture = Some(Gesture::Redo),
                                _ => {}
                            }
                        }
                    }
                }
            }
            out.push(event);
            return;
        }
        match std::mem::take(&mut self.state) {
            State::Idle => match event {
                Event::PointerButton { pos, button: PointerButton::Primary, pressed: true, modifiers }
                    if self.touches.len() == 1 && self.pencils.is_disjoint(&self.touches) =>
                {
                    self.state = State::Pending { pos, modifiers, start: now, buffered: Vec::new() };
                }
                other => out.push(other),
            },
            State::Pending { pos, modifiers, start, mut buffered } => match event {
                Event::PointerMoved(p) if p.distance(pos) <= SLOP => {
                    buffered.push(event);
                    self.state = State::Pending { pos, modifiers, start, buffered };
                }
                Event::PointerMoved(_) | Event::PointerGone | Event::PointerButton { .. } => {
                    // moved away, lifted (a tap) or cancelled: let egui see what really happened
                    Self::release(pos, modifiers, buffered, out);
                    out.push(event);
                }
                other => {
                    // keys, scroll, zoom…: not part of the gesture
                    out.push(other);
                    self.state = State::Pending { pos, modifiers, start, buffered };
                }
            },
            State::Swallow => match event {
                Event::PointerButton { button: PointerButton::Primary, pressed: false, .. } | Event::PointerGone => {
                    // the gesture is over
                    if matches!(event, Event::PointerGone) {
                        out.push(event);
                    }
                }
                other => {
                    out.push(other);
                    self.state = State::Swallow;
                }
            },
        }
    }

    /// Drop the held press (a second finger landed): egui never sees it, nor the release that
    /// ends it. The small moves held with it are dropped too (the pointer is still where they say).
    fn drop_pending(&mut self) {
        if matches!(self.state, State::Pending { .. }) {
            self.state = State::Swallow;
        }
    }

    fn release(pos: Pos2, modifiers: Modifiers, buffered: Vec<Event>, out: &mut Vec<Event>) {
        out.push(Event::PointerButton { pos, button: PointerButton::Primary, pressed: true, modifiers });
        out.extend(buffered);
    }

    /// Start time of a held press, for tests.
    #[cfg(test)]
    fn backdate(&mut self, by: Duration) {
        if let State::Pending { start, .. } = &mut self.state
            && let Some(t) = start.checked_sub(by)
        {
            *start = t;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{TouchDeviceId, TouchId};

    fn touch(id: u64, phase: TouchPhase, x: f32) -> Event {
        Event::Touch { device_id: TouchDeviceId(0), id: TouchId(id), phase, pos: Pos2::new(x, 10.0), force: None }
    }
    fn button(b: PointerButton, pressed: bool, x: f32) -> Event {
        Event::PointerButton { pos: Pos2::new(x, 10.0), button: b, pressed, modifiers: Modifiers::NONE }
    }
    fn is_button(e: &Event, b: PointerButton, p: bool) -> bool {
        matches!(e, Event::PointerButton { button, pressed, .. } if *button == b && *pressed == p)
    }

    #[test]
    fn tap_passes_through_as_press_and_release() {
        let mut lp = LongPress::default();
        let mut ev = vec![touch(1, TouchPhase::Start, 10.0), Event::PointerMoved(Pos2::new(10.0, 10.0)), button(PointerButton::Primary, true, 10.0)];
        let wake = lp.filter(&mut ev, Instant::now());
        assert!(wake.is_some(), "a held press asks for a wake-up");
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Primary, true)), "press is held back");
        let mut ev = vec![touch(1, TouchPhase::End, 10.0), button(PointerButton::Primary, false, 10.0), Event::PointerGone];
        lp.filter(&mut ev, Instant::now());
        let press = ev.iter().position(|e| is_button(e, PointerButton::Primary, true));
        let release = ev.iter().position(|e| is_button(e, PointerButton::Primary, false));
        assert!(matches!((press, release), (Some(p), Some(r)) if p < r), "{ev:?}");
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Secondary, true)));
    }

    #[test]
    fn still_finger_becomes_secondary_click_and_release_is_dropped() {
        let mut lp = LongPress::default();
        let mut ev = vec![touch(1, TouchPhase::Start, 10.0), button(PointerButton::Primary, true, 10.0), Event::PointerMoved(Pos2::new(13.0, 10.0))];
        lp.filter(&mut ev, Instant::now());
        lp.backdate(HOLD);
        let mut ev = Vec::new();
        assert_eq!(lp.filter(&mut ev, Instant::now()), None);
        assert!(ev.iter().any(|e| is_button(e, PointerButton::Secondary, true)));
        assert!(ev.iter().any(|e| is_button(e, PointerButton::Secondary, false)));
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Primary, true)));
        let mut ev = vec![touch(1, TouchPhase::End, 10.0), button(PointerButton::Primary, false, 10.0), Event::PointerGone];
        lp.filter(&mut ev, Instant::now());
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Primary, false)), "{ev:?}");
        // and the next tap works normally again
        let mut ev = vec![
            touch(2, TouchPhase::Start, 50.0),
            button(PointerButton::Primary, true, 50.0),
            touch(2, TouchPhase::End, 50.0),
            button(PointerButton::Primary, false, 50.0),
        ];
        lp.filter(&mut ev, Instant::now());
        assert!(ev.iter().any(|e| is_button(e, PointerButton::Primary, true)));
        assert!(ev.iter().any(|e| is_button(e, PointerButton::Primary, false)));
    }

    #[test]
    fn drag_releases_the_press_before_the_move() {
        let mut lp = LongPress::default();
        let mut ev = vec![touch(1, TouchPhase::Start, 10.0), button(PointerButton::Primary, true, 10.0), Event::PointerMoved(Pos2::new(40.0, 10.0))];
        assert_eq!(lp.filter(&mut ev, Instant::now()), None);
        let press = ev.iter().position(|e| is_button(e, PointerButton::Primary, true));
        let moved = ev.iter().position(|e| matches!(e, Event::PointerMoved(p) if p.x > 30.0));
        assert!(matches!((press, moved), (Some(p), Some(m)) if p < m), "{ev:?}");
    }

    #[test]
    fn second_finger_cancels_the_long_press() {
        let mut lp = LongPress::default();
        let mut ev = vec![touch(1, TouchPhase::Start, 10.0), button(PointerButton::Primary, true, 10.0), touch(2, TouchPhase::Start, 60.0)];
        assert_eq!(lp.filter(&mut ev, Instant::now()), None);
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Primary, true)), "the held press is dropped: {ev:?}");
        lp.backdate(HOLD);
        let mut ev = Vec::new();
        lp.filter(&mut ev, Instant::now());
        assert!(ev.is_empty());
        // nor does its release reach egui
        let mut ev = vec![touch(1, TouchPhase::End, 10.0), button(PointerButton::Primary, false, 10.0), touch(2, TouchPhase::End, 60.0)];
        lp.filter(&mut ev, Instant::now());
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Primary, false)), "{ev:?}");
    }

    /// A quick tap of `n` fingers, each moving `wander` points before lifting. Whether a primary
    /// release reached egui.
    fn multi_tap(lp: &mut LongPress, n: u64, wander: f32, held: Duration) -> bool {
        let t0 = Instant::now();
        let mut ev: Vec<Event> = (1..=n).map(|i| touch(i, TouchPhase::Start, 100.0 * i as f32)).collect();
        ev.insert(1, button(PointerButton::Primary, true, 100.0));
        lp.filter(&mut ev, t0);
        let mut ev: Vec<Event> = (1..=n).map(|i| touch(i, TouchPhase::Move, 100.0 * i as f32 + wander)).collect();
        lp.filter(&mut ev, t0);
        let mut ev: Vec<Event> = (1..=n).map(|i| touch(i, TouchPhase::End, 100.0 * i as f32 + wander)).collect();
        ev.insert(1, button(PointerButton::Primary, false, 100.0));
        lp.filter(&mut ev, t0 + held);
        ev.iter().any(|e| is_button(e, PointerButton::Primary, false))
    }

    #[test]
    fn two_finger_tap_is_undo_and_three_is_redo() {
        let quick = Duration::from_millis(150);
        let mut lp = LongPress::default();
        assert!(!multi_tap(&mut lp, 2, 2.0, quick), "no click reaches egui");
        assert_eq!(lp.take_gesture(), Some(Gesture::Undo));
        assert_eq!(lp.take_gesture(), None, "taken once");
        assert!(!multi_tap(&mut lp, 3, 0.0, quick));
        assert_eq!(lp.take_gesture(), Some(Gesture::Redo));
        // a pinch or pan (fingers wander), a slow rest, or a one-finger tap: nothing
        assert!(!multi_tap(&mut lp, 2, 40.0, quick));
        assert_eq!(lp.take_gesture(), None);
        assert!(!multi_tap(&mut lp, 2, 0.0, Duration::from_millis(900)));
        assert_eq!(lp.take_gesture(), None);
        let mut ev = vec![touch(9, TouchPhase::Start, 10.0), touch(9, TouchPhase::End, 10.0)];
        lp.filter(&mut ev, Instant::now());
        assert_eq!(lp.take_gesture(), None);
        // a finger beside the resting Pencil is not a tap
        lp.mark_pencil(1);
        let _ = multi_tap(&mut lp, 2, 0.0, quick);
        assert_eq!(lp.take_gesture(), None);
    }

    #[test]
    fn pencil_press_goes_through_at_once_and_never_long_presses() {
        let mut lp = LongPress::default();
        lp.mark_pencil(7);
        let mut ev = vec![touch(7, TouchPhase::Start, 10.0), button(PointerButton::Primary, true, 10.0)];
        assert_eq!(lp.filter(&mut ev, Instant::now()), None);
        assert!(ev.iter().any(|e| is_button(e, PointerButton::Primary, true)), "{ev:?}");
        lp.backdate(HOLD);
        let mut ev = Vec::new();
        lp.filter(&mut ev, Instant::now());
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Secondary, true)), "{ev:?}");
        let mut ev = vec![touch(7, TouchPhase::End, 10.0), button(PointerButton::Primary, false, 10.0)];
        lp.filter(&mut ev, Instant::now());
        assert!(ev.iter().any(|e| is_button(e, PointerButton::Primary, false)), "the release reaches egui");
        // the same id later used by a finger is held back again (winit reuses UITouch addresses)
        let mut ev = vec![touch(7, TouchPhase::Start, 10.0), button(PointerButton::Primary, true, 10.0)];
        assert!(lp.filter(&mut ev, Instant::now()).is_some());
        assert!(!ev.iter().any(|e| is_button(e, PointerButton::Primary, true)));
    }

    #[test]
    fn mouse_presses_are_not_held() {
        let mut lp = LongPress::default();
        let mut ev = vec![button(PointerButton::Primary, true, 10.0)];
        assert_eq!(lp.filter(&mut ev, Instant::now()), None);
        assert_eq!(ev.len(), 1);
    }
}
