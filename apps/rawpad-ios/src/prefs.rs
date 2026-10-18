//! The interface settings (`UiState`: panels, language, appearance…) kept between launches, like
//! the desktop's `ui.json`. They live in `Library/Application Support/RawPad UI.json`, next to the
//! library folder: backed up with the app, not shown in Files.
//!
//! A damaged file is set aside as `RawPad UI.json.corrupt-<unix time>` and defaults are used; a
//! file that can't be read at all is never overwritten this session.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use lightcraft_ui_egui::UiState;

/// How often changed settings are written while the app runs (and always when it goes to the
/// background or quits).
const EVERY: Duration = Duration::from_secs(3);

pub fn path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").filter(|h| !h.is_empty())?;
    Some(PathBuf::from(home).join("Library").join("Application Support").join("RawPad UI.json"))
}

/// The saved settings, a message for the user when the file couldn't be used, and whether the
/// file must be left alone this session.
pub fn load(path: &Path) -> (Option<UiState>, Option<String>, bool) {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return (None, None, false),
        Err(e) => return (None, Some(format!("The interface settings couldn't be read ({e}); defaults are used.")), true),
    };
    match decode(&bytes) {
        Ok(ui) => (Some(ui.sanitized()), None, false),
        Err(e) => {
            let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            let aside = path.with_file_name(format!("RawPad UI.json.corrupt-{secs}"));
            let kept = std::fs::rename(path, &aside);
            log::warn!("interface settings damaged ({e}); set aside: {kept:?}");
            (None, Some(format!("The interface settings were damaged ({e}); defaults are used.")), kept.is_err())
        }
    }
}

/// Parse saved settings; a panel layout this version can't use is dropped, not the whole file.
fn decode(bytes: &[u8]) -> Result<UiState, String> {
    let mut saved: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let fields = saved.as_object_mut().ok_or("settings must be an object")?;
    let bad_docking =
        fields.get("docking").is_some_and(|v| match serde_json::from_value::<Option<lightcraft_ui_egui::docking::Workspace>>(v.clone()) {
            Ok(Some(w)) => w.validate().is_err(),
            Ok(None) => false,
            Err(_) => true,
        });
    if bad_docking {
        fields.remove("docking");
    }
    serde_json::from_value(saved).map_err(|e| e.to_string())
}

/// Writes the settings when they change (checked every few seconds) and on demand.
pub struct Writer {
    path: Option<PathBuf>,
    written: Vec<u8>,
    checked: Instant,
    keep_file: bool,
    failing: bool,
}

impl Writer {
    pub fn new(path: Option<PathBuf>, current: &UiState, keep_file: bool) -> Self {
        Self { path, written: serde_json::to_vec_pretty(current).unwrap_or_default(), checked: Instant::now(), keep_file, failing: false }
    }

    /// Called every frame; writes at most every [`EVERY`].
    pub fn tick(&mut self, ui: &UiState) {
        if self.checked.elapsed() < EVERY {
            return;
        }
        self.checked = Instant::now();
        self.save(ui);
    }

    /// Write now if anything changed (background, quit, memory warning).
    pub fn save(&mut self, ui: &UiState) {
        if self.keep_file {
            return;
        }
        let Some(path) = self.path.as_deref() else { return };
        if ui.docking.as_ref().is_some_and(|w| w.validate().is_err()) {
            return;
        }
        let Ok(bytes) = serde_json::to_vec_pretty(ui) else { return };
        if bytes == self.written {
            return;
        }
        match write_atomic(path, &bytes) {
            Ok(()) => {
                self.written = bytes;
                self.failing = false;
            }
            Err(e) => {
                if !self.failing {
                    log::error!("saving the interface settings failed: {e}");
                }
                self.failing = true;
            }
        }
    }
}

/// A temp file, fsynced, renamed over the old one: never a half-written file.
fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    let written = (|| {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()
    })();
    if let Err(e) = written {
        let _ = std::fs::remove_file(&tmp);
        return Err(e);
    }
    std::fs::rename(&tmp, path)
}
