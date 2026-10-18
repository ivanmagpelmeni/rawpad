//! The iPadOS host: one UIKit window (winit), egui-winit for input, egui-wgpu on Metal for drawing,
//! and the LightCraft app (`lightcraft_ui_egui::LightcraftApp`) running every frame.
//!
//! Environment (set by CI through `SIMCTL_CHILD_…`; unset on the iPad):
//! - `RAWPAD_DEMO=1`: a new library starts with the procedural demo photos.
//! - `RAWPAD_CONTROL=<port>`: start the JSON-lines control server on 127.0.0.1 (CI only).

use std::num::NonZeroU32;
use std::path::PathBuf;
use std::sync::Arc;
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

/// Wakes the event loop from other threads (finished renders, control requests).
#[derive(Debug, Clone, Copy)]
enum UserEvent {
    RepaintAfter(Duration),
}

pub fn run() {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info)).ok();
    log::info!("RawPad {} starting", env!("CARGO_PKG_VERSION"));
    let event_loop = match EventLoop::<UserEvent>::with_user_event().build() {
        Ok(l) => l,
        Err(e) => {
            log::error!("could not create the event loop: {e}");
            return;
        }
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut host = Host { proxy: event_loop.create_proxy(), ctx: egui::Context::default(), gfx: None, app: None, next_repaint: None };
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
    match s.open_library(&dir, seed_demo) {
        Ok(r) => log::info!("library {}: {} photos ({} log records replayed)", dir.display(), s.catalog.len(), r.replayed),
        Err(e) => log::error!("can't open library {}: {e}", dir.display()),
    }
    s
}

impl Host {
    fn start(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let attrs = Window::default_attributes().with_title("RawPad");
        let window = Arc::new(event_loop.create_window(attrs).map_err(|e| format!("create window: {e}"))?);
        let config = egui_wgpu::WgpuConfiguration::default();
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
            let mut app = LightcraftApp::new(open_session(), Services::default());
            lightcraft_ui_egui::i18n::set_language(app.ui.language);
            if let Some(port) = std::env::var("RAWPAD_CONTROL").ok().and_then(|p| p.parse::<u16>().ok()) {
                let rx = control_server::start(port, self.ctx.clone());
                app = app.with_control(rx);
            }
            if app.session.catalog.is_empty() {
                lightcraft_ui_egui::panels::empty_library::open_on_empty_library(&mut app);
            }
            self.app = Some(app);
        }
        window.request_redraw();
        self.gfx = Some(Gfx { window, input, painter });
        Ok(())
    }

    fn frame(&mut self) {
        let (Some(gfx), Some(app)) = (self.gfx.as_mut(), self.app.as_mut()) else { return };
        let mut raw = gfx.input.take_egui_input(&gfx.window);
        app.raw_input_hook(&mut raw);
        let mut out = self.ctx.run_ui(raw, |ui| {
            app.logic(ui.ctx());
            app.ui(ui);
        });
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
        // the library keeps every command in its journal; also catch state changed outside commands
        app.session.persist_if_dirty();
        if let Some(vp) = out.viewport_output.get(&ViewportId::ROOT) {
            self.schedule(vp.repaint_delay);
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

/// Log lines go to stderr (the simulator's console in CI, Console.app on a Mac).
struct StderrLogger;
static LOGGER: StderrLogger = StderrLogger;

impl log::Log for StderrLogger {
    fn enabled(&self, m: &log::Metadata<'_>) -> bool {
        m.level() <= log::Level::Info
    }
    fn log(&self, r: &log::Record<'_>) {
        if self.enabled(r.metadata()) {
            eprintln!("[rawpad {} {}] {}", r.level(), r.target(), r.args());
        }
    }
    fn flush(&self) {}
}
