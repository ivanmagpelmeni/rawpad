//! The iPadOS host: one UIKit window (winit), egui-winit for input, egui-wgpu on Metal for drawing,
//! and the LightCraft app (`lightcraft_ui_egui::LightcraftApp`) running every frame.
//!
//! Environment (set by CI through `SIMCTL_CHILD_…`; unset on the iPad):
//! - `RAWPAD_DEMO=1`: a new library starts with the procedural demo photos.
//! - `RAWPAD_CONTROL=<port>`: start the JSON-lines control server on 127.0.0.1 (CI only).

use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use egui::ViewportId;
use lightcraft_engine::Session;
use lightcraft_ui_egui::{LightcraftApp, Services};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

// The desktop app's loopback control server, used as is (it only needs std, egui and the UI crate).
#[path = "../../lightcraft/src/control_server.rs"]
#[allow(dead_code)]
mod control_server;

// The desktop app's logger, used as is: standard error plus a rotated log file (see `logs_dir`).
#[path = "../../lightcraft/src/logging.rs"]
#[allow(dead_code)]
mod logging;

/// Wakes the event loop from other threads (finished renders, control requests).
#[derive(Debug, Clone, Copy)]
enum UserEvent {
    RepaintAfter(Duration),
}

pub fn run() {
    install_logger();
    log::info!("RawPad {} starting", env!("CARGO_PKG_VERSION"));
    let event_loop = match EventLoop::<UserEvent>::with_user_event().build() {
        Ok(l) => l,
        Err(e) => {
            log::error!("could not create the event loop: {e}");
            return;
        }
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut host = Host {
        proxy: event_loop.create_proxy(),
        ctx: egui::Context::default(),
        gfx: None,
        app: None,
        next_repaint: None,
        long_press: Default::default(),
        logo_key: false,
        keys_logged: 0,
        keys: None,
        text_field: None,
        field_held: None,
        keep_in_view: 0,
        holding: false,
        last_tap: None,
        frames: 0,
        last_logged_field: None,
        soft_keyboard: false,
        logged_insets: None,
        avoid: Default::default(),
        logged_cover: (0, 0),
        last_cover: 0,
        prefs: None,
        share_offer: Default::default(),
    };
    if let Err(e) = event_loop.run_app(&mut host) {
        log::error!("event loop ended with an error: {e}");
    }
}

struct Gfx {
    window: Arc<Window>,
    input: egui_winit::State,
    painter: egui_wgpu::winit::Painter,
}

struct Host {
    proxy: EventLoopProxy<UserEvent>,
    ctx: egui::Context,
    gfx: Option<Gfx>,
    app: Option<LightcraftApp>,
    next_repaint: Option<Instant>,
    long_press: crate::touch::LongPress,
    /// ⌘ held on a hardware keyboard (winit's logo key), for `keys::apply`.
    logo_key: bool,
    /// Key presses logged so far (only the first few, so typed text doesn't fill the log).
    keys_logged: u8,
    /// Hardware-keyboard presses read from UIKit (winit 0.30 passes none on iOS; CI run 30).
    keys: Option<crate::native::Keys>,
    /// The egui text box being edited last frame (`PlatformOutput::ime`), if any.
    text_field: Option<egui::Rect>,
    /// The text box the keyboard is up for, kept while it is focused but out of view
    /// (`keys::hold_field`, DECISIONS 67).
    field_held: Option<(egui::Id, egui::Rect)>,
    /// Frames left in which the focused field scrolls itself into view after the UI lifted
    /// (`text_field::KEEP_IN_VIEW`, DECISIONS 67).
    keep_in_view: u8,
    /// `field_held` is standing in for a box out of view (logged once per stretch).
    holding: bool,
    /// When and where the last tap or press was (a finger, the Pencil or the CI's pointer).
    last_tap: Option<(Instant, u64, egui::Pos2)>,
    /// Frames drawn so far (for the tap window, `keys::tap_age`).
    frames: u64,
    /// The text box last frame, to log a refused keyboard once per field.
    last_logged_field: Option<egui::Rect>,
    /// The on-screen keyboard is up (`keys::soft_keyboard`, DECISIONS.md 47).
    soft_keyboard: bool,
    logged_insets: Option<crate::native::EdgeInsets>,
    /// Keyboard avoidance: shrinks the UI while the on-screen keyboard covers a focused field.
    avoid: crate::keyboard::Avoid,
    /// The last keyboard height and UI shrink logged (whole points), so the log shows changes only.
    logged_cover: (i32, i32),
    /// The keyboard height of the last frame (whole points).
    last_cover: i32,
    prefs: Option<crate::prefs::Writer>,
    /// The Share button for the files of the last export.
    share_offer: crate::share_offer::ShareOffer,
}

/// `<app container>/Library/Application Support/RawPad Library` (backed up with the app, not shown in Files).
fn library_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").filter(|h| !h.is_empty())?;
    Some(PathBuf::from(home).join("Library").join("Application Support").join("RawPad Library"))
}

fn open_session() -> Session {
    let fresh = || Session::new().with_fs().with_system_clock();
    let seed_demo = std::env::var("RAWPAD_DEMO").is_ok_and(|v| v == "1");
    let Some(dir) = library_dir() else {
        log::error!("no HOME: the library is kept in memory only");
        return if seed_demo { Session::with_demo().with_fs().with_system_clock() } else { fresh() };
    };
    if let Err(e) = std::fs::create_dir_all(&dir) {
        log::error!("can't create the library folder {}: {e}", dir.display());
    }
    let mut s = fresh();
    // Export ▸ Add to Apple Photos saves to the Photos app, on a worker thread
    s.apple_photos = Some(crate::native::photos_runner());
    s.apple_photos_background = true;
    match s.open_library(&dir, seed_demo).map(|r| r.replayed) {
        Ok(replayed) => log::info!("library {}: {} photos ({replayed} log records replayed)", dir.display(), s.catalog.len()),
        Err(e) => log::error!("can't open library {}: {e}", dir.display()),
    }
    s
}

/// Room kept at the top for iPadOS's multitasking "•••" button. The status bar is hidden, so
/// UIKit reports a top inset of 0 and the button sat over the search box (CI run 21).
const MIN_TOP_INSET: f32 = 20.0;

/// Frames (about half a second) in which a focused field scrolls itself into view after the UI
/// lifted above the on-screen keyboard (DECISIONS 67).
const KEEP_IN_VIEW_FRAMES: u8 = 30;

impl Host {
    fn start(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let attrs = Window::default_attributes().with_title("RawPad");
        let window = Arc::new(event_loop.create_window(attrs).map_err(|e| format!("create window: {e}"))?);
        let mut config = egui_wgpu::WgpuConfiguration::default();
        // Ask for what the GPU actually has: egui-wgpu's default asks for WebGPU's default limits,
        // which the iOS simulator's Metal doesn't meet (max_inter_stage_shader_variables 15 < 16).
        if let egui_wgpu::WgpuSetup::CreateNew(create) = &mut config.wgpu_setup {
            create.device_descriptor = Arc::new(|adapter: &wgpu::Adapter| wgpu::DeviceDescriptor {
                label: Some("rawpad egui device"),
                required_limits: adapter.limits(),
                ..Default::default()
            });
        }
        let mut painter = pollster::block_on(egui_wgpu::winit::Painter::new(self.ctx.clone(), config, false, egui_wgpu::RendererOptions::default()));
        pollster::block_on(painter.set_window(ViewportId::ROOT, Some(Arc::clone(&window)))).map_err(|e| format!("Metal surface: {e}"))?;
        let input =
            egui_winit::State::new(self.ctx.clone(), ViewportId::ROOT, &window, Some(window.scale_factor() as f32), None, painter.max_texture_side());
        if self.app.is_none() {
            // a second OS window (the second-display view) isn't available: draw it inside this one
            self.ctx.set_embed_viewports(true);
            let proxy = self.proxy.clone();
            self.ctx.set_request_repaint_callback(move |info| {
                // the loop may be gone while the app quits: nothing to wake then
                let _ = proxy.send_event(UserEvent::RepaintAfter(info.delay));
            });
            let services = {
                let (window, ctx) = (Arc::clone(&window), self.ctx.clone());
                let links = ctx.clone();
                let (share_window, share_ctx) = (Arc::clone(&window), ctx.clone());
                Services {
                    picker: Some(Box::new(move |req| crate::native::pick(&window, &ctx, &req))),
                    // exports write files themselves (the background exporter needs the shared one)
                    write_shared: Some(Arc::new(lightcraft_engine::export::write_file)),
                    write: Some(Box::new(lightcraft_engine::export::write_file)),
                    png: Some(Box::new(|img: &lightcraft_raster::Rgba8| {
                        lightcraft_codecs::encode_png(&lightcraft_codecs::EncodeImage::rgba8(img), &lightcraft_codecs::EncodeMeta::default())
                            .unwrap_or_default()
                    })),
                    // Help links open in Safari (egui-winit hands them to UIApplication)
                    open_url: Some(Box::new(move |url: &str| {
                        if !url.starts_with("https://") {
                            return Err("only https links are opened".into());
                        }
                        links.open_url(egui::OpenUrl::new_tab(url));
                        Ok(())
                    })),
                    // every "Show in Finder" (a photo's original, a folder, Help › Open Log Folder)
                    // opens the share sheet: Save to Files, AirDrop, Mail…
                    reveal: Some(Box::new(move |path: &str| {
                        let zoom = f64::from(share_ctx.zoom_factor());
                        let at = share_ctx.input(|i| i.pointer.latest_pos()).map(|p| (f64::from(p.x) * zoom, f64::from(p.y) * zoom));
                        crate::native::share(&share_window, path, at)
                    })),
                    log_file: LOG_FILE.get().map(|p| p.to_string_lossy().into_owned()),
                    ..Services::default()
                }
            };
            let mut app = LightcraftApp::new(open_session(), services);
            // the interface as it was left (panels, language, appearance…)
            let prefs_path = crate::prefs::path();
            let (saved, warning, keep) = prefs_path.as_deref().map_or((None, None, false), crate::prefs::load);
            if let Some(ui) = saved {
                app.ui = ui;
            }
            app.notices.extend(warning);
            self.prefs = Some(crate::prefs::Writer::new(prefs_path, &app.ui, keep));
            lightcraft_ui_egui::i18n::set_language(app.ui.language);
            if let Some(port) = std::env::var("RAWPAD_CONTROL").ok().and_then(|p| p.parse::<u16>().ok()) {
                let rx = control_server::start(port, self.ctx.clone());
                app = app.with_control(rx);
            }
            if app.session.catalog.is_empty() {
                lightcraft_ui_egui::panels::empty_library::open_on_empty_library(&mut app);
            }
            // First export goes to Documents/Exports (Files ▸ On My iPad ▸ RawPad), not to the
            // desktop default ~/Pictures, which the Files app can't see inside the sandbox.
            if app.session.last_export.is_none()
                && let Some(dir) = crate::native::exports_dir()
            {
                let _ = std::fs::create_dir_all(&dir);
                app.session.last_export = Some(serde_json::json!({ "dir": dir.to_string_lossy() }));
            }
            self.app = Some(app);
        }
        self.keys = crate::native::Keys::attach(&window, &self.ctx);
        if self.keys.is_none() {
            log::warn!("keyboard (UIKit): no key view, hardware keys reach only winit");
        }
        window.request_redraw();
        self.gfx = Some(Gfx { window, input, painter });
        Ok(())
    }

    fn frame(&mut self) {
        let (Some(gfx), Some(app)) = (self.gfx.as_mut(), self.app.as_mut()) else { return };
        let mut raw = gfx.input.take_egui_input(&gfx.window);
        let mut keep_drawing = false;
        // Keep the UI out from under the status bar and the home indicator. egui-winit's own
        // insets came through as nothing (CI run 8: the bottom toolbar sat on the home
        // indicator), so read UIKit's.
        if let Some(rect) = raw.screen_rect {
            if let Some(i) = crate::native::safe_area(&gfx.window) {
                if self.logged_insets != Some(i) {
                    log::info!("safe area (points): top {} left {} bottom {} right {}", i.top, i.left, i.bottom, i.right);
                    self.logged_insets = Some(i);
                }
                // UIKit points → egui points (egui's zoom factor, 1 unless changed in the UI)
                let zoom = self.ctx.zoom_factor().max(0.1);
                let (t, l, b, r) = ((i.top as f32 / zoom).max(MIN_TOP_INSET), i.left as f32 / zoom, i.bottom as f32 / zoom, i.right as f32 / zoom);
                // the docked on-screen keyboard, beyond the safe area it sits on
                let cover = crate::native::keyboard_cover(&gfx.window).map_or(0.0, |c| (c - i.bottom).max(0.0) as f32 / zoom);
                let focused = self.ctx.memory(|m| m.focused()).filter(|_| self.ctx.egui_wants_keyboard_input());
                let field_bottom = focused.and_then(|id| self.ctx.read_response(id)).map(|r| r.rect.bottom());
                let was_shrunk = self.avoid.active();
                let kb = self.avoid.inset(cover, focused, field_bottom, rect.max.y - b);
                if self.avoid.active() && !was_shrunk {
                    // the field may now lie below its shrunk panel: scroll it into view
                    self.keep_in_view = KEEP_IN_VIEW_FRAMES;
                    // the CI tour checks for this line (a field low on the screen, DECISIONS.md 66)
                    log::info!(
                        "keyboard: lifting the UI above the on-screen keyboard (field bottom {field_bottom:?}, keyboard top {:.0})",
                        rect.max.y - b - cover
                    );
                }
                // log the keyboard once it has settled (same height two frames running); a new
                // shrink at the same height is logged too (run 67 logged 257 pt only once)
                let whole = cover.round() as i32;
                if whole == self.last_cover && (whole, kb.round() as i32) != self.logged_cover {
                    self.logged_cover = (whole, kb.round() as i32);
                    log::info!("keyboard (UIKit): on-screen keyboard covers {cover:.0} pt, UI shrunk by {kb:.0} pt");
                }
                // the keyboard slides in and out over a few frames: keep drawing until it settles
                keep_drawing = whole != 0 || self.last_cover != 0 || self.avoid.active();
                self.last_cover = whole;
                let inner = egui::Rect::from_min_max(egui::pos2(rect.min.x + l, rect.min.y + t), egui::pos2(rect.max.x - r, rect.max.y - b - kb));
                if inner.is_positive() {
                    raw.screen_rect = Some(inner);
                }
            } else if let Some(insets) = raw.safe_area_insets {
                raw.screen_rect = Some(rect - insets);
            }
        }
        // Imports copy into the library: picked files are temporary copies, and folders from
        // Files or an SD card stay readable only while the app runs.
        if let Some(scan) = app.scan.as_mut() {
            scan.copy = true;
        }
        // long-press → secondary click (context menus); before the hook, so control-channel
        // pointer events (CI) are never held back
        let long_press_in = self.long_press.filter(&mut raw.events, Instant::now());
        // a quick two-finger tap undoes, a three-finger tap redoes (DECISIONS.md 49); not while
        // a text box is being edited
        if let Some(gesture) = self.long_press.take_gesture()
            && self.text_field.is_none()
        {
            let id = match gesture {
                crate::touch::Gesture::Undo => "edit.undo",
                crate::touch::Gesture::Redo => "edit.redo",
            };
            match app.run(id, serde_json::json!({})) {
                Ok(_) => log::info!("touch: multi-finger tap → {id}"),
                Err(e) => log::info!("touch: multi-finger tap → {id} refused: {e}"),
            }
        }
        // ⌘ is `command` on the iPad as on the Mac (egui-winit maps it to Ctrl off macOS)
        // (UIKit's own presses come with their modifiers already Mac-style: added after)
        // the on-screen keyboard: only for a text box being edited (last frame) that took the
        // focus right after a tap on it, never for a focused slider, the photo or a mask (Ivan's iPad,
        // 2026-10-19: `egui_wants_keyboard_input` is also true for a tapped slider)
        self.frames = self.frames.wrapping_add(1);
        let frames = self.frames;
        let tap =
            self.last_tap.map(|(t, frame, at)| (crate::keys::tap_age(Instant::now().saturating_duration_since(t), frames.saturating_sub(frame)), at));
        let was_up = self.soft_keyboard;
        self.soft_keyboard = crate::keys::soft_keyboard(was_up, self.text_field, tap);
        if !self.soft_keyboard && self.text_field.is_some() && self.last_logged_field != self.text_field {
            // once per focused field: why the keyboard stayed down (CI run 41)
            log::info!("keyboard: text field {:?} focused without a tap on it (last tap {tap:?}): on-screen keyboard stays down", self.text_field);
        }
        self.last_logged_field = self.text_field;
        let text_focus = self.soft_keyboard;
        let (uikit_events, uikit_cmd) = self
            .keys
            .as_ref()
            .map(|k| {
                k.focus(text_focus);
                k.drain()
            })
            .unwrap_or_default();
        crate::keys::apply(&mut raw.events, self.logo_key || uikit_cmd);
        raw.events.extend(uikit_events);
        app.raw_input_hook(&mut raw);
        // (after the hook: the CI's pointer events count as taps too)
        if let Some(at) = crate::keys::tap_pos(&raw.events) {
            self.last_tap = Some((Instant::now(), self.frames, at));
        }
        let keep_in_view = self.keep_in_view > 0;
        self.keep_in_view = self.keep_in_view.saturating_sub(1);
        self.ctx.data_mut(|d| d.insert_temp(egui::Id::new(lightcraft_ui_egui::text_field::KEEP_IN_VIEW), keep_in_view));
        let share_offer = &mut self.share_offer;
        let mut share: Option<(Vec<String>, egui::Pos2)> = None;
        let mut out = self.ctx.run_ui(raw, |ui| {
            if keep_drawing {
                ui.ctx().request_repaint_after(Duration::from_millis(100));
            }
            app.logic(ui.ctx());
            app.ui(ui);
            // after an export: a Share button for the files just written, above the export toast
            let ctx = ui.ctx();
            let now = ctx.input(|i| i.time);
            share_offer.update(app.last_export_result.as_ref(), now);
            let area = app.canvas_rect.unwrap_or_else(|| ctx.content_rect());
            if let Some(shown) =
                share_offer.show(ctx, egui::pos2(area.center().x, area.bottom() - 90.0), now, lightcraft_ui_egui::i18n::language().code())
            {
                app.widgets.push((crate::share_offer::WIDGET_ID.to_owned(), shown.rect));
                if let Some(files) = shown.share {
                    share = Some((files, shown.rect.center()));
                }
            }
        });
        // only an editable, focused TextEdit sets `ime` (egui's IME/caret output). Its `rect` is
        // only the text cursor's (zero wide in an empty box, CI run 42), so the box is the focused
        // widget's whole rectangle, with the cursor's for good measure
        let focused_rect = self.ctx.memory(|m| m.focused()).and_then(|id| self.ctx.read_response(id)).map(|r| r.rect);
        let ime_field = out.platform_output.ime.as_ref().map(|ime| focused_rect.map_or(ime.rect, |r| r.union(ime.rect)));
        // a focused box out of view (just after the UI lifted) keeps the keyboard (CI run 69)
        let focused_id = self.ctx.memory(|m| m.focused());
        let held = crate::keys::hold_field(ime_field, focused_id, self.soft_keyboard, self.field_held);
        let holding = ime_field.is_none() && held.is_some();
        if holding && !self.holding {
            // the CI tour checks for this line (DECISIONS 67)
            log::info!("keyboard: focused text field out of view, on-screen keyboard kept up");
        }
        self.holding = holding;
        self.field_held = held;
        self.text_field = ime_field.or(held.map(|(_, r)| r));
        gfx.input.handle_platform_output(&gfx.window, std::mem::take(&mut out.platform_output));
        let clipped = self.ctx.tessellate(std::mem::take(&mut out.shapes), out.pixels_per_point);
        gfx.painter.paint_and_update_textures(
            ViewportId::ROOT,
            out.pixels_per_point,
            [0.0, 0.0, 0.0, 1.0],
            &clipped,
            &mut out.textures_delta,
            Vec::new(),
            &gfx.window,
        );
        if let Some((files, at)) = share {
            let zoom = f64::from(self.ctx.zoom_factor());
            let paths: Vec<&str> = files.iter().map(String::as_str).collect();
            if let Err(e) = crate::native::share_files(&gfx.window, &paths, Some((f64::from(at.x) * zoom, f64::from(at.y) * zoom))) {
                log::warn!("share after export: {e}");
                app.toast_error(&self.ctx, e);
            }
        }
        // the library keeps every command in its journal; also catch state changed outside commands
        app.session.persist_if_dirty();
        if let Some(p) = self.prefs.as_mut() {
            p.tick(&app.ui);
        }
        if let Some(vp) = out.viewport_output.get(&ViewportId::ROOT) {
            self.schedule(vp.repaint_delay);
        }
        if let Some(delay) = long_press_in {
            self.schedule(delay);
        }
    }

    fn schedule(&mut self, delay: Duration) {
        if delay == Duration::MAX {
            return;
        }
        let at = Instant::now().checked_add(delay).unwrap_or_else(Instant::now);
        self.next_repaint = Some(self.next_repaint.map_or(at, |t| t.min(at)));
    }

    /// Write everything now: iPadOS may end a backgrounded app without another callback.
    fn save(&mut self) {
        if let Some(app) = self.app.as_mut() {
            app.session.persist_if_dirty();
            app.session.save_view();
            if let Some(p) = self.prefs.as_mut() {
                p.save(&app.ui);
            }
        }
    }
}

impl ApplicationHandler<UserEvent> for Host {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gfx.is_some() {
            if let Some(g) = &self.gfx {
                g.window.request_redraw();
            }
            return;
        }
        if let Err(e) = self.start(event_loop) {
            log::error!("RawPad could not start: {e}");
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        let UserEvent::RepaintAfter(delay) = event;
        if delay.is_zero() {
            if let Some(g) = &self.gfx {
                g.window.request_redraw();
            }
        } else {
            self.schedule(delay);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match &event {
            // winit gives an altitude angle only for Apple Pencil touches; egui drops it, so note
            // them here for the long-press filter (a resting Pencil tip must draw, not open a menu)
            WindowEvent::Touch(t) if matches!(t.force, Some(winit::event::Force::Calibrated { altitude_angle: Some(_), .. })) => {
                self.long_press.mark_pencil(t.id);
            }
            WindowEvent::ModifiersChanged(m) => {
                self.logo_key = m.state().super_key();
                log::info!("keyboard: modifiers {:?}", m.state());
            }
            // logged so a device log shows whether a hardware keyboard reaches the app at all
            WindowEvent::KeyboardInput { event: k, .. } if self.keys_logged < 6 => {
                self.keys_logged = self.keys_logged.saturating_add(1);
                log::info!("keyboard: {:?} {:?}", k.logical_key, k.state);
            }
            _ => {}
        }
        let Some(gfx) = self.gfx.as_mut() else { return };
        let response = gfx.input.on_window_event(&gfx.window, &event);
        match event {
            WindowEvent::RedrawRequested => self.frame(),
            WindowEvent::Resized(size) => {
                if let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                    gfx.painter.on_window_resized(ViewportId::ROOT, w, h);
                }
                gfx.window.request_redraw();
            }
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                self.save();
                event_loop.exit();
            }
            _ => {
                if response.repaint {
                    gfx.window.request_redraw();
                }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match self.next_repaint {
            Some(t) if t <= Instant::now() => {
                self.next_repaint = None;
                if let Some(g) = &self.gfx {
                    g.window.request_redraw();
                }
                event_loop.set_control_flow(ControlFlow::Wait);
            }
            Some(t) => event_loop.set_control_flow(ControlFlow::WaitUntil(t)),
            None => event_loop.set_control_flow(ControlFlow::Wait),
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.save();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.save();
        if let Some(app) = self.app.as_mut()
            && let Err(e) = app.session.close_library()
        {
            log::error!("saving the library failed: {e}");
        }
    }

    fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {
        log::warn!("memory warning: saving and returning freed memory");
        self.save();
        lightcraft_sysmem::release_free_memory();
    }
}

/// `<app container>/Documents/Logs`: shown in Files › On My iPad › RawPad › Logs, so a log can be
/// shared after a crash. Each launch moves the previous log to `lightcraft.1.log` (then `.2`).
fn logs_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").filter(|h| !h.is_empty())?;
    Some(PathBuf::from(home).join("Documents").join("Logs"))
}

/// The current log file, once [`install_logger`] has opened it (Help › Open Log Folder shares it).
static LOG_FILE: OnceLock<PathBuf> = OnceLock::new();

/// Log lines go to stderr (the simulator's console in CI, Console.app on a Mac) and to the log
/// file in [`logs_dir`]; panics that escape are added to the file too.
fn install_logger() {
    static LOGGER: OnceLock<logging::AppLogger> = OnceLock::new();
    let filter = logging::Filter::parse("warn,lightcraft*=info,rawpad*=info");
    let max = filter.max();
    let logger = LOGGER.get_or_init(|| logging::AppLogger::new(filter, true));
    if log::set_logger(logger).is_err() {
        return;
    }
    log::set_max_level(max);
    logging::record_panics(logger);
    match logs_dir() {
        Some(dir) => match logger.attach_dir(&dir) {
            Ok(path) => {
                log::info!("log file {}", path.display());
                let _ = LOG_FILE.set(path);
            }
            Err(e) => log::warn!("no log file, standard error only: {e}"),
        },
        None => logger.no_file(),
    }
}
