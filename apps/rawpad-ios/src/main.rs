//! RawPad: the LightCraft photo library and raw developer as a native iPadOS app.
//!
//! The same egui UI as the desktop app (`crates/ui-egui`), hosted by winit (UIKit) and drawn by
//! egui-wgpu on Metal. Everything iPad-specific lives in this crate; on other targets the binary
//! is a stub so the upstream workspace builds unchanged. See `IPAD.md` at the repo root.
// `unsafe` is denied everywhere except `native` (UIKit calls; see DECISIONS.md).
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
mod native;
// touch gestures are plain egui events, so their tests also run on the CI's macOS host
#[cfg(any(target_os = "ios", test))]
mod touch;

fn main() {
    #[cfg(target_os = "ios")]
    ios::run();
    #[cfg(not(target_os = "ios"))]
    eprintln!("rawpad runs on iPadOS only: build it with --target aarch64-apple-ios or aarch64-apple-ios-sim (see IPAD.md)");
}
