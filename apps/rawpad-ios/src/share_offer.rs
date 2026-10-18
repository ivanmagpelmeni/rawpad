//! After an export: a **Share…** button for the files just written, so they can go to AirDrop,
//! Mail or Save to Files without looking for them in Files › On My iPad › RawPad › Exports.
//!
//! The host watches `LightcraftApp::last_export_result` (set when an export batch finishes). A new
//! result with written files puts the button above the export toast for [`SHOW_FOR`] seconds; the
//! host opens the share sheet for those files when it is tapped. Plain egui and JSON, so the tests
//! also run on the CI's macOS host.

use std::time::Duration;

use egui::{Context, Pos2, Rect};
use serde_json::Value;

/// How long the button stays up after an export, in seconds.
pub const SHOW_FOR: f64 = 20.0;
/// The button's automation id (control channel `ui.clickWidget`, `ui.widgets`).
#[cfg_attr(not(target_os = "ios"), allow(dead_code))]
pub const WIDGET_ID: &str = "button:rawpadShareExport";

/// The files an export result says were written (`files[].path`; skipped and failed ones have none).
pub fn exported_paths(result: &Value) -> Vec<String> {
    result
        .get("files")
        .and_then(Value::as_array)
        .map(|files| files.iter().filter_map(|f| f.get("path").and_then(Value::as_str)).filter(|p| !p.is_empty()).map(str::to_owned).collect())
        .unwrap_or_default()
}

/// The button's label in the UI's language (`n` files). Kept here rather than in upstream's
/// catalogs (`crates/ui-egui/locales`), which the iPad port doesn't patch for its own strings.
/// Plural-heavy languages put the count in brackets so one form fits every number.
pub fn label(language: &str, n: usize) -> String {
    let (one, many) = match language {
        "de" => ("Exportiertes Foto teilen…", "{n} exportierte Fotos teilen…"),
        "fr" => ("Partager la photo exportée…", "Partager les {n} photos exportées…"),
        "es" => ("Compartir foto exportada…", "Compartir {n} fotos exportadas…"),
        "it" => ("Condividi foto esportata…", "Condividi {n} foto esportate…"),
        "pt-br" => ("Compartilhar foto exportada…", "Compartilhar {n} fotos exportadas…"),
        "ru" => ("Поделиться экспортированным фото…", "Поделиться экспортированными фото ({n})…"),
        "uk" => ("Поділитися експортованим фото…", "Поділитися експортованими фото ({n})…"),
        "pl" => ("Udostępnij wyeksportowane zdjęcie…", "Udostępnij wyeksportowane zdjęcia ({n})…"),
        "cs" => ("Sdílet exportovanou fotografii…", "Sdílet exportované fotografie ({n})…"),
        "ka" => ("ექსპორტირებული ფოტოს გაზიარება…", "ექსპორტირებული ფოტოების ({n}) გაზიარება…"),
        "ja" => ("書き出した写真を共有…", "書き出した写真 {n} 枚を共有…"),
        "zh-hans" => ("共享导出的照片…", "共享导出的 {n} 张照片…"),
        "zh-hant" => ("分享匯出的照片…", "分享匯出的 {n} 張照片…"),
        _ => ("Share Exported Photo…", "Share {n} Exported Photos…"),
    };
    if n == 1 { one.to_owned() } else { many.replace("{n}", &n.to_string()) }
}

/// What [`ShareOffer::show`] did this frame.
#[derive(Debug, Clone, PartialEq)]
pub struct Shown {
    /// The Share button's rectangle (egui points).
    pub rect: Rect,
    /// Tapped this frame: share these files.
    pub share: Option<Vec<String>>,
}

#[derive(Debug, Default)]
pub struct ShareOffer {
    /// The last export result looked at (a new one replaces the offer).
    seen: Option<Value>,
    paths: Vec<String>,
    until: f64,
}

impl ShareOffer {
    /// Look at the app's latest export result (`now` in egui seconds).
    pub fn update(&mut self, result: Option<&Value>, now: f64) {
        if result == self.seen.as_ref() {
            return;
        }
        self.seen = result.cloned();
        self.paths = result.map(exported_paths).unwrap_or_default();
        self.until = now + SHOW_FOR;
    }

    /// The files on offer right now (none once the time is up or the offer was used/dismissed).
    pub fn paths(&self, now: f64) -> &[String] {
        if now <= self.until { &self.paths } else { &[] }
    }

    pub fn dismiss(&mut self) {
        self.paths.clear();
    }

    /// Draw the offer centred above `bottom_center`. Returns `None` when there is nothing to offer.
    /// `language` is the UI's language code (`lightcraft_ui_egui::i18n::language().code()`).
    pub fn show(&mut self, ctx: &Context, bottom_center: Pos2, now: f64, language: &str) -> Option<Shown> {
        let n = self.paths(now).len();
        if n == 0 {
            return None;
        }
        let label = label(language, n);
        let (share, close) = egui::Area::new(egui::Id::new("rawpad-share-export"))
            .order(egui::Order::Foreground)
            .pivot(egui::Align2::CENTER_BOTTOM)
            .fixed_pos(bottom_center)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let share = ui.add(egui::Button::new(label).min_size(egui::vec2(0.0, 32.0)));
                            let close = ui.add(egui::Button::new("×").min_size(egui::vec2(32.0, 32.0)));
                            (share, close)
                        })
                        .inner
                    })
                    .inner
            })
            .inner;
        let tapped = share.clicked();
        let files = tapped.then(|| self.paths.clone());
        if tapped || close.clicked() {
            self.dismiss();
        }
        // come back to take it down when the time is up
        ctx.request_repaint_after(Duration::from_secs_f64((self.until - now).clamp(0.05, SHOW_FOR)));
        Some(Shown { rect: share.rect, share: files })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn written_files_only() {
        let r = json!({"files": [{"path": "/x/a.jpg"}, {"skipped": "/x/b.jpg"}, {"error": "disk full"}, {"path": ""}], "cancelled": false});
        assert_eq!(exported_paths(&r), vec!["/x/a.jpg".to_owned()]);
        assert!(exported_paths(&json!({"files": "nonsense"})).is_empty());
        assert!(exported_paths(&json!(null)).is_empty());
    }

    #[test]
    fn a_new_result_offers_its_files_for_a_while() {
        let mut o = ShareOffer::default();
        o.update(None, 0.0);
        assert!(o.paths(0.0).is_empty());
        let r = json!({"files": [{"path": "/x/a.jpg"}, {"path": "/x/b.jpg"}]});
        o.update(Some(&r), 10.0);
        assert_eq!(o.paths(10.0).len(), 2);
        // the same result again doesn't bring a dismissed offer back
        o.dismiss();
        o.update(Some(&r), 11.0);
        assert!(o.paths(11.0).is_empty());
        // a new export does, and the offer lapses after SHOW_FOR
        let r2 = json!({"files": [{"path": "/x/c.jpg"}]});
        o.update(Some(&r2), 12.0);
        assert_eq!(o.paths(12.0), ["/x/c.jpg".to_owned()]);
        assert!(o.paths(12.0 + SHOW_FOR + 0.1).is_empty());
    }

    #[test]
    fn labels_follow_the_language_and_count() {
        assert_eq!(label("en", 1), "Share Exported Photo…");
        assert_eq!(label("en", 3), "Share 3 Exported Photos…");
        assert_eq!(label("ru", 5), "Поделиться экспортированными фото (5)…");
        assert_eq!(label("de", 1), "Exportiertes Foto teilen…");
        // an unknown code falls back to English; no form keeps the placeholder
        assert_eq!(label("xx", 2), "Share 2 Exported Photos…");
        for code in ["en", "de", "fr", "es", "it", "pt-br", "ru", "uk", "pl", "cs", "ka", "ja", "zh-hans", "zh-hant"] {
            assert!(!label(code, 7).contains("{n}") && label(code, 7).contains('7'), "{code}");
        }
    }

    /// One frame; its font texture isn't uploaded anywhere (epaint panics on unhandled deltas).
    fn frame(ctx: &Context, input: egui::RawInput, f: impl FnMut(&mut egui::Ui)) {
        let mut out = ctx.run_ui(input, f);
        out.textures_delta.clear();
    }

    #[test]
    fn tapping_share_hands_over_the_files_once() {
        let ctx = Context::default();
        let mut o = ShareOffer::default();
        o.update(Some(&json!({"files": [{"path": "/x/a.jpg"}]})), 0.0);
        let mut shown = None;
        // a new area measures itself (invisibly) on its first frame
        for _ in 0..3 {
            frame(&ctx, egui::RawInput::default(), |ui| shown = o.show(ui.ctx(), egui::pos2(400.0, 500.0), 0.0, "en"));
        }
        let shown = shown.unwrap();
        assert!(shown.share.is_none());
        assert!(shown.rect.width() > 0.0 && shown.rect.max.y <= 500.5);
        // a click in the middle of the button
        let at = shown.rect.center();
        let mut input = egui::RawInput::default();
        input.events = vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() },
            egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() },
        ];
        let mut tapped = None;
        frame(&ctx, input, |ui| tapped = o.show(ui.ctx(), egui::pos2(400.0, 500.0), 1.0, "en"));
        assert_eq!(tapped.unwrap().share, Some(vec!["/x/a.jpg".to_owned()]));
        assert!(o.paths(1.0).is_empty());
    }
}
