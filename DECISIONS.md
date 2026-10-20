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

## 2026-10-18: Nightly session 2 (19:54 Berlin)

24. **The on-device log is upstream's desktop logger, compiled in by `#[path]`** (like the
    control server): stderr plus `Documents/Logs/lightcraft.log`, rotated each launch
    (`lightcraft.1.log`, `.2`), panics recorded. In `Documents` (not `Library/Caches`) so Ivan can
    find and share it in Files › On My iPad › RawPad › Logs. Filter `warn,lightcraft*=info,rawpad*=info`.
25. **AI items are also hidden in the mask Add/Subtract/Intersect menus on iOS** (Object,
    Describe…, Sky, Subject), patched in `masking.rs::component_menu`.
26. **The ArtCraft Discord/website links stay for now.** They credit and point to upstream's
    community; hiding them needs patches in five upstream files (topbar, dialogs, menus, menubar,
    links). The brand rule only covers the ArtCraft marks/logos, which are already removed.
    Revisit if Ivan asks.
27. **"Add to Apple Photos" reuses the engine's Photos import with an iOS runner**
    (`native::photos_runner`, `PHPhotoLibrary performChangesAndWait` on the engine's worker
    thread) instead of a new iPad-only export path. Two small upstream gates opened for iOS (see
    `UPSTREAM.md`). No albums on iPad (the field is hidden). The share sheet is still to do.
28. **Risky native work is tested on a branch first:** push the branch, start the iPad workflow
    on it with `workflow_dispatch` (release off; releases only come from main), merge into main
    only when that run is green and its replies/screenshots look right.

## 2026-10-18: Nightly session 3 (20:55 Berlin)

29. **The share sheet is the iPad's `Services::reveal`.** Every upstream *Show in Finder* place
    (Photo › Show in Finder for an original raw, a folder's context menu, Help › Open Log Folder)
    opens `UIActivityViewController` for that file (`native::share`), labelled **Share…**
    (`reveal_label` patched for iOS). This also covers "export the original raw to Files"
    (*Save to Files* in the sheet) and sending the log, without a new iPad-only command. The
    popover is anchored at the last pointer position (window points), else the window's middle.
    `Services::log_file` is now set on iOS so Help › Open Log Folder is enabled.
30. **The 24 MP GPU fallback is treated as simulator-only for now.** The engine's GPU device
    (`crates/gpu/src/ctx.rs`) already asks for `adapter.limits()`; the 256 MiB storage-buffer
    limit is what the simulator's Metal reports. Real Apple GPUs report far larger buffers. No
    tiling work until a device log shows the fallback ("storage-buffer limit" in
    `lightcraft.log`).

## 2026-10-18: Nightly session 4 (21:55 Berlin)

31. **"Share Exported Photo…" is drawn by the host, not patched into upstream.** The host watches
    `LightcraftApp::last_export_result` (set when a background export, i.e. the Export dialog,
    finishes) and shows a small button above the export toast for 20 s (`src/share_offer.rs`);
    tapping it opens one share sheet for all files written (`native::share_files`). Registered in
    `app.widgets` as `button:rawpadShareExport` so the tour can click it. Synchronous control-channel
    exports (no `background`) don't set that result, so they don't offer a share. The label is
    English only for now (the module stays free of the UI crate so it unit-tests on macOS).
32. **The tour waits instead of sleeping where it can:** `sim_test.py` has a `waitUntil` step
    (repeat a request until a dotted path in its result is truthy; fails the tour on timeout). Used
    for the background export (`export.last`) and the Photos import (`imports.imported`).

## 2026-10-18: Nightly session 5 (22:54 Berlin)

33. **Add to Photos uses the asynchronous `performChanges:completionHandler:`** and the runner's
    thread waits on a channel for the answer, at most the runner's limit (`timeout_for`, 10 min +
    5 s a file). `performChangesAndWait:error:` never returned when called from the export's
    worker thread (the Export dialog's path; run 19), although it worked from the engine's
    `apple-photos` thread. With the async call the same tour step saved the photo in under a second
    (run 20). No answer in time is reported as the import's timeout, never a stuck "Adding to Apple
    Photos…" row. The runner still refuses the main thread (it waits).

## 2026-10-18: Nightly session 6 (23:55 Berlin)

34. **Pinch and pan are tested with egui's own events.** The tour sends `ui.zoom` (egui's pinch
    event), a two-finger scroll and a one-finger drag over the loupe and checks `ui.zoom` and
    `ui.pan` in `ui.inspect` (`sim_test.py`'s `waitUntil` got a `differs` option). Real fingers
    reach the same upstream code (`detail.rs::navigate_gesture`) through egui's multi-touch input;
    a pinch frame skips the one-finger drag-pan (`if !navigating`), so the two don't add up. The
    simulator can't inject real touches without extra tools (idb), so the real-finger check is on
    the iPad (Needs Ivan).
35. **20 pt kept at the top** (`ios.rs::MIN_TOP_INSET`) for iPadOS's multitasking "•••" button:
    with the status bar hidden UIKit reports a top inset of 0 and the button covered the search box.
36. **The host's own strings get a small host-side table** (`share_offer::label`, the 14 upstream
    languages, chosen by `i18n::language().code()`) instead of patching upstream's locale catalogs.
    Slavic and Georgian forms put the count in brackets, so one form fits every number.
37. **CI warms up the simulator's Photos library** (`xcrun simctl addmedia` of the app icon right
    after boot) and waits up to 240 s for the background export: on a cold simulator "Add to Apple
    Photos" took 92 s (run 24) or longer (run 22) instead of under a second (runs 20, 21). Not a
    hang in the app: the photo was saved, just late.

## 2026-10-19: Nightly session 7 (00:54 Berlin)

38. **The CI tour's Add to Apple Photos step only warns on a timeout** (`"soft": true` in
    `sim_test.py`'s `waitUntil`) and runs last. Same app code passed in runs 20, 21, 25 and 28 and
    hung for 240 s in runs 22, 24 and 26 (no permission prompt on screen; the warm-up of
    decision 37 didn't help), so it's the simulator's photolibraryd, not the app. The Share button
    check now uses a background export without Photos, so it no longer depends on it. The app
    logs the Photos permission status before each save (`permission status 3` = allowed) to tell
    a missing permission from a slow library on the iPad.
39. **Apple Pencil touches are never held back for a long-press.** winit reports an altitude angle
    only for Pencil touches (`Force::Calibrated { altitude_angle: Some(_) }`); egui-winit drops
    it, so the host marks those touch ids before passing the event on (`touch::LongPress::
    mark_pencil`). A resting Pencil tip starts a brush stroke instead of opening a context menu.
    Fingers keep the 500 ms long-press. The Pencil path can't be driven in the simulator: real
    check on the iPad (Needs Ivan).
40. **⌘ on a hardware keyboard is egui's `command`, as on the Mac** (`src/keys.rs`): egui-winit
    maps `command` to Ctrl outside macOS, so the host tracks winit's logo key and rewrites the
    modifiers of the frame's events. Whether winit delivers iPad hardware keys at all is probed in
    CI (osascript keystrokes into Simulator.app on the CI Mac, never failing) and logged on the
    device (first six key presses and modifier changes, `keyboard:` lines in `lightcraft.log`).

## 2026-10-19: Nightly session 8 (01:54 Berlin)

41. **RawPad reads hardware-keyboard presses from UIKit itself** (`native::KeyView`): a 1×1,
    input-transparent UIView inside winit's view holds first responder and turns
    `pressesBegan:`/`pressesEnded:` (`UIKey.keyCode` = USB HID usage, `modifierFlags`,
    `characters`) into egui key and text events (`keys::uikit_events`, unit-tested on the CI Mac).
    Reason: run 30 showed no winit `KeyboardInput` at all (winit 0.30's iOS view doesn't seem to
    handle presses). The view takes first responder back every frame, but never while a dialog
    (Files picker, share sheet) is presented, so their own text fields keep the keyboard. It
    swallows the presses (no `super` call), so winit can't deliver them twice. The run 30 probe
    ran under an open share sheet and proved nothing; the probe now runs before any sheet.
42. **Modifier flags are tracked per press, clearing a modifier's bit on its own release**
    (`keys::uikit_flags_after`): UIKit reports the released ⌘ with `UIKeyModifierCommand` still
    set (run 33), which would leave ⌘ held for every later tap.

## 2026-10-19: Nightly session 9 (02:55 Berlin)

43. **On-screen keyboard through a second key view** (`native::TextKeyView`, adopts `UIKeyInput`):
    it becomes first responder only while an egui text field has focus
    (`ctx.egui_wants_keyboard_input()`, last frame), so UIKit shows the keyboard then and only
    then; otherwise `KeyView` (no `UIKeyInput`) holds it as before, so no keyboard or shortcut bar
    appears while browsing. `insertText:` → egui text (Return = Enter, Tab = Tab),
    `deleteBackward` → Backspace (`keys::soft_text_events`, `soft_backspace`; unit-tested).
    `hasText` is always true so delete always reaches the field. Both views swallow hardware
    presses and share one channel and one modifier state, so hardware keys never arrive twice.
    Chosen over an `inputView` swap on one view, which would leave the iPad's shortcut bar up all
    the time.
44. **Keypad digits are number keys.** CI run 35 showed osascript's `5` arriving as HID 0x5D
    (keypad 5, `UIKeyModifierNumericPad`), which the HID table dropped, so the rating never changed
    and the ⌘Z check passed only because nothing had changed. Keypad 0x59–0x62 now map to
    Num1…Num0 (plus keypad `/` and `.`).

## 2026-10-20: Morning session (08:40 Berlin)

45. **Keyboard avoidance shrinks egui's screen rect, but only for a covered field**
    (`src/keyboard.rs`, `native::keyboard_cover`). The docked keyboard's height comes from the
    view's `keyboardLayoutGuide` (iOS 15+, RawPad needs 17; no notification observer needed); a
    floating or undocked keyboard leaves the guide on the safe area, so nothing shrinks. The UI
    shrinks only when the focused field's bottom edge (last frame) would sit under the keyboard,
    and that decision is kept for the field until it loses focus or the keyboard goes, because
    after shrinking the field moves up and re-deciding would make the layout jump. A field near
    the top (search box) leaves the layout alone. The host repaints every 100 ms while the
    keyboard is up or moving so the layout follows its slide. Safe-area insets are now divided by
    egui's zoom factor (UIKit points → egui points), a no-op at the default zoom.
46. **The tour's hardware-keyboard rating checks are hard checks again** (no `"soft"`): runs 37
    and 38 both passed them. Typing into the search box (`ui.inspect` → `copied`) starts soft.
