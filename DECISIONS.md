# Decisions

Dated architecture and product decisions, each with the reason and the rejected alternatives.
Newer entries win over older ones and over the kickoff brief.

## 2026-10-18: Kickoff

1. **Approach A: a native port of the existing UI.**
   - `lightcraft-ui-egui` is hosted by our own crate (`apps/rawpad-ios`), using winit 0.30 (UIKit),
     egui-winit and egui-wgpu on Metal.
   - Why: `ui-egui` depends only on `egui`, and all host services are injected through
     `lightcraft_ui_egui::Services` (the web build already proves a second host works). The product
     is pure Rust, including TLS, so no C toolchain problems. wgpu 30 includes Metal. This is a
     true port of every function.
   - Rejected:
     - **B, a SwiftUI shell over FFI:** about 77k lines of UI to rewrite, not a port.
     - **C, a WKWebView around the WASM build:** WebGL instead of Metal, browser storage iPadOS can
       evict, and no Pencil input. It stays the fallback only with Ivan's explicit OK.
2. **No eframe on iOS.**
   - eframe 0.36 has no iOS support, so the host drives `egui::Context::run_ui` itself. That is the
     same loop `crates/ui-egui/src/headless.rs` and eframe use: `raw_input_hook`, then `logic`,
     then `ui`.
3. **No Xcode project, for now.**
   - `cargo build` produces the binary. `ios/build.sh` assembles the `.app` from `ios/Info.plist`
     and the icons, ad-hoc signs it for the simulator, and zips an unsigned `.ipa` for the device.
   - Why: fewer moving parts, everything lives as text in git, nothing to generate.
   - Rejected for now: an XcodeGen project. Revisit it if we need Swift code, storyboards, asset
     catalogs or entitlements; it is the planned fallback.
4. **Scope v1 = full upstream parity, except:**
   - AI masks (SAM 3) and AI denoise, because the models are several GB. Built without the `sam`
     and `denoise` engine features. Manual noise reduction stays.
   - The second-display window: embedded viewports instead
     (`egui::Context::set_embed_viewports(true)`).
   - The control channel and MCP as user features: CI uses the control channel only.
   - Hardware limits:
     - SDR display
     - the USB-C Pencil has no pressure, so brushes use fixed flow
     - about 6 GB RAM on the A16, so heavy merges are tiled or batch-limited
5. **Device target: iPad 11th gen (A16), iPadOS 26.**
   - `MinimumOSVersion` is 17.0, so builds also run on older simulators.
   - iPad only: `UIDeviceFamily` is 2.
6. **Where the data lives.**
   - The library is at `<container>/Library/Application Support/RawPad Library`: backed up, and
     hidden from Files so it can't be damaged there.
   - `UIFileSharingEnabled` is on, so `Documents/` (used for exports) shows in Files under
     *On My iPad › RawPad*.
   - Imports are copied into the library (Ivan's choice). Security-scoped bookmarks to outside
     folders are not used in v1.
7. **Saving.**
   - Every command is journaled by the engine.
   - The host also calls `persist_if_dirty` and `save_view` every frame and on `suspended`,
     `memory_warning` and `exiting`.
   - `exiting` also calls `close_library` for a full snapshot.
   - On `memory_warning` the host also calls `lightcraft_sysmem::release_free_memory`.
8. **Versioning.**
   - The app version is `1.<CI run number>` (`CFBundleShortVersionString`), and the build is the
     run number. Each release is unique, so SideStore always sees updates.
   - The upstream version is named in the release notes.
9. **Releases and the SideStore source.**
   - GitHub Release tag `v1.<run>` with `RawPad.ipa`.
   - `sidestore/source.json` is committed to main by CI. Source URL:
     `https://raw.githubusercontent.com/Ivanmagpelmeni/rawpad/main/sidestore/source.json`.
10. **CI.**
    - One `macos-15` standard runner job: fmt and clippy (`-D warnings`) for our crate, a
      simulator build, an iPad simulator tour with screenshots, then the device `.ipa`.
    - A small `ubuntu-latest` job publishes the release.
    - Upstream's desktop and packaging workflows were deleted: they would run on every upstream
      merge and test nothing we ship.
11. **The upstream control server is reused, not copied.**
    - It is compiled into our crate with `#[path = "../../lightcraft/src/control_server.rs"]`, so
      upstream fixes arrive with merges.
    - It starts only when `RAWPAD_CONTROL=<port>` is set (CI). It listens on loopback only.
12. **Sessions.**
    - Every Sunday, one scheduled session per hour from 18:54 to 02:54 Berlin time (9 runs, the last ends around 03:40; changed by Ivan on 2026-10-18).
    - Each run works about 45 minutes, then writes `HANDOFF.md` and pushes.
    - The next run starts by reading `HANDOFF.md`.
    - Ivan gives feedback in chat; Claude records it in `FEEDBACK.md`.
13. **Sandbox limit.** Claude's sandbox can't reach crates.io or `static.rust-lang.org` (proxy
    policy), so `cargo check` for iOS can't run in-session. CI is the compiler. GitHub works through
    the session's GitHub connection.

## 2026-10-18: First CI results and Files access

14. **The egui device asks for the adapter's own limits.**
    - The iOS simulator's Metal reports `max_inter_stage_shader_variables` = 15, below WebGPU's
      default of 16, so egui-wgpu's default request failed (black screen).
    - We now use `required_limits: adapter.limits()`.
15. **`unsafe` lives in `apps/rawpad-ios/src/native.rs` only.**
    - It holds the UIKit calls through objc2 0.6 / objc2-ui-kit 0.3.
    - `main.rs` denies `unsafe_code`; `native.rs` allows it, with a `// SAFETY:` note on each
      block.
    - Rejected: a separate helper crate (upstream's sysmem pattern). The crate is already
      iOS-only and private, so a second crate would add ceremony without isolating anything new.
16. **Files access goes through `Services::picker` (UIDocumentPickerViewController).**
    - **Photos** (`file.addPhotos`) are picked `asCopy`, so the app gets temporary copies and no
      lasting permission is needed.
    - **Folders** (`file.addFolder`, export folders: SD card, Files) are opened in place, and
      their security scope is kept for the app's lifetime.
    - **Save requests** write to `Documents/Exports` without a dialog.
    - The synchronous `pick_folder` and `pick_files` services stay unset, so buttons inside
      dialogs that rely on them are hidden.
17. **Imports always start as Copy into the library.** The host sets `ScanTask::copy` each
    frame. Picked copies are temporary and folder access ends with the app, so "Add in place"
    would lose originals.
18. **Exports go to `Documents/Exports` by default,** shown in Files › On My iPad › RawPad. The
    host seeds `Session::last_export` instead of patching upstream's `default_export_dir`
    (`~/Pictures`, invisible in Files).
19. **Safe area.** The host shrinks `RawInput::screen_rect` by egui-winit's iOS safe-area insets.
    The status bar is hidden (`UIViewControllerBasedStatusBarAppearance` = false).

## 2026-10-18: First nightly session (18:54 Berlin)

20. **Long-press → secondary click is a host-side gesture filter** (`src/touch.rs`), run on
    `RawInput::events` before `raw_input_hook`. A touch's primary press is held back until the
    finger lifts (tap), moves more than 10 pt (drag/scroll), a second finger lands (pinch), or
    500 ms pass (secondary click at the landing spot; the following primary release is dropped
    so it can't close the new menu). Mouse/trackpad presses and control-channel events are never
    held. Rejected: relying on egui's `long_touched` (unclear in the 0.36 popup code, and the
    primary release would still close the menu); patching upstream's context-menu calls.
    Trade-off: a still finger gets no press feedback for up to 500 ms.
21. **Safe area comes from UIKit, not egui-winit** (supersedes how #19 gets the insets). CI run 8
    showed the bottom toolbar on the home indicator, so egui-winit's insets were empty; the host
    reads `UIView.safeAreaInsets` with one `msg_send!` in `native.rs` (own `UIEdgeInsets`
    struct, no new objc2 features) and falls back to egui's insets.
22. **Interface settings (`UiState`) persist in `Library/Application Support/RawPad UI.json`**
    (own small writer in `src/prefs.rs`, modelled on the desktop's `PrefsWriter`, not shared:
    that one lives in the desktop binary). Checked every 3 s and written on background/quit;
    damaged files are set aside as `.corrupt-<time>`. `settings.library_path` in it is ignored:
    the iPad always opens `RawPad Library`.
23. **The iOS crate's unit tests run on the macOS CI host** (`cargo test -p rawpad-ios`; egui is a
    dev-dependency so pure-egui modules like `touch.rs` compile there).
