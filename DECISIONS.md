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

## 2026-10-19: Editing audit session (evening, unattended; pushed 2026-10-20)

Written on 2026-10-19 as decisions 45–50; renumbered 47–52 when merged after the 2026-10-20
morning session, which had taken 45 and 46.

47. **The on-screen keyboard comes up only for a text box that was tapped** (`keys::soft_keyboard`,
    `keys::tap_pos`). Ivan's iPad showed it while editing: `ctx.egui_wants_keyboard_input()` (used
    since decision 43) is true for *any* focused widget, and every slider takes the focus on a tap
    or drag (`widgets.rs` `slider`: `resp.request_focus()`), so moving a slider brought the
    keyboard up. Now the host uses egui's `PlatformOutput::ime` (set only while an editable
    `TextEdit` has the focus; its `rect` is just the text cursor, so the box is the focused
    widget's rectangle, CI run 42) and brings the keyboard up only when that box took the focus
    within 0.8 s or 4 frames of a tap that landed on it (8 pt slop; frames because the simulator
    draws slowly and a real iPad may wait for the cursor blink, CI run 41). It stays up while a text
    box keeps the focus and goes when none has it. A box focused by code after a tap elsewhere (the
    mask Effect menu's search, a dialog's name field) leaves it down until tapped. Typing a slider
    value still works: tapping the number opens a text box in its place, under the finger. A
    hardware keyboard is unchanged (`KeyView` turns presses into text, decision 41). Keyboard
    avoidance (decision 45) only acts while the keyboard is up, so it follows the same rule.
48. **Every Before/After view is reachable by touch:** on iOS a chevron button right of the bottom
    bar's Before/After button opens a menu (Side by Side, Split, Top/Bottom, Split Top/Bottom, Show
    Original; picking the active one turns it off). The button alone only toggles Side by Side; the
    other views were behind Shift+Y, Alt+Y, Alt+Shift+Y, `\` and the View menu. Upstream's split
    line sits at the middle of the photo and can't be dragged on the desktop either, so no
    draggable divider was added. Small patch in `panels/bottombar.rs` (UPSTREAM.md).
49. **Two-finger tap = Undo, three-finger tap = Redo** (as in Procreate and other iPad apps):
    every finger lifts within 350 ms of the first landing and none moves more than 10 pt; not while
    a text box is being edited; a Pencil touch spoils it. Runs `edit.undo` / `edit.redo`
    (`touch::Gesture`, host). Before this, undo needed Menu › Edit › Undo (three taps). Also: a
    second finger landing now *drops* the first finger's held press instead of passing it to egui,
    so a pinch or a multi-finger tap can't leave a stray brush dot, spot or click.
50. **Touch scrolling of the Edit panel no longer moves sliders:** on iOS a drag on a slider track
    that sets off more vertically than sideways scrolls the panel (`widgets.rs` `touch_scrolls`).
    And the Tool Overlay's Auto mode always shows mask and spot pins on iOS (`detail.rs`
    `pins_live`): Auto showed them only while a pointer hovered the photo, which a lifted finger
    never does. Both are small `cfg!(target_os = "ios")` patches (UPSTREAM.md).
51. **Ivan installs with AltStore Classic, not SideStore:** his iPad is managed by an MDM that
    blocks VPN apps, so SideStore's LocalDevVPN can't run. AltServer runs on his Windows PC; he
    installs, updates and refreshes over a USB-C cable (PC and iPad are on different networks, so
    Wi-Fi refresh doesn't work). SETUP.md has a section for it. The source now lists RawPad's
    privacy permission (`NSPhotoLibraryAddUsageDescription`) in `appPermissions.privacy`, read
    from `ios/Info.plist` by `update_source.py`: AltStore 2.x checks that the source declares every
    usage description and entitlement of the .ipa and refuses to install on a mismatch
    (faq.altstore.io, "Make a Source"). Entitlements stay empty (the unsigned .ipa carries none).
52. **The audit session could not push** (the repo wasn't attached with write access). Its commits
    went to the claude.ai Project as a patch series (`claude/rawpad-audit-2026-10-19.mbox`) and
    were applied unchanged (blob hashes checked) in an interactive session on 2026-10-20, then
    merged with the morning session's keyboard avoidance on branch `audit-2026-10-19`. Scheduled
    tasks must attach the repo with push access (their prompts already say so).
53. **2026-10-25: Touch grab radius 22 pt on iPad** (about a fingertip; Apple's minimum target is
    44 pt across) for tone-curve points, crop handles and mask pins/handles; the desktop keeps
    upstream's 10–12 pt. The nearest point or handle still wins when several are in reach. A crop
    drag on iPad starts at the press (`press_origin()`), not where the finger is once egui's drag
    threshold has passed (the curve already did this). All `cfg!(target_os = "ios")` constants in
    upstream files (UPSTREAM.md).
54. **2026-10-25: Delete a curve point by long-press** (the curve's context menu gets *Delete
    Point* for the inner point under the press), not a bigger double-tap target: a double-tap on a
    point that just moved a little would add a point instead. iOS only, plus `cfg!(test)` so the
    upstream unit test (`tests_curve.rs`) can check it.
55. **2026-10-25: CI runs upstream UI tests that cover our patches**
    (`cargo test -p lightcraft-ui-egui --lib tests_curve`), so patched upstream code gets a
    compiled test too. Add more test module names to that step as more patches get tests.
56. **2026-10-25: A Done button in the Crop panel on iPad** (`button:cropDone`, runs `tool.done`),
    next to Reset: the crop was applied only by a double-tap inside it or Return.
57. **2026-10-25: The last small touch targets on iPad.** The radial mask's rotate knob sits 44 pt
    (a fingertip) out from the right-edge handle on screen, along the mask's axis, and is drawn
    with a 7 pt radius on a dark fill (desktop: 0.08 of the long side, 4 pt); the drag sets the
    angle from the centre to the finger, so the distance doesn't change what it does. Rating stars
    keep their drawn size (19–20 pt) but take a touch area of 36 pt tall plus the gaps between them
    (`widgets::stars`). The targeted adjustment samples where the finger landed (`press_origin`).
58. **2026-10-25: Picking several photos by touch = *Select Several* in the photo's long-press
    menu** (grid, filmstrip, loupe), not a new bottom-bar button: it needs no space in an already
    full bar and stays out of the way for one-photo work. While it is on, a tap adds a photo to the
    selection or takes it out (as ⌘-click), and a long-press no longer replaces the selection; the
    same menu says *Done Selecting* to turn it off. The flag is egui temp data
    (`rawpad.selectSeveral`), so it resets at each launch. CI's tour turns it on and checks that two
    taps make 3 selected. No on-screen indicator yet besides the highlighted cells.
59. **2026-10-25: An on-screen strip while *Select Several* is on.** Under the grid's title row a
    40 pt strip reads "Tap photos to add or remove them" with a *Done* button
    (`button:selectSeveralDone`) that turns it off; the count stays in the title row ("3 selected ·
    24 photos"). Only in the grid: the loupe's filmstrip has no room, and its long-press menu still
    says *Done Selecting*. CI's tour taps two photos that are fully on screen (`clickVisible` in
    `ios/sim_test.py`), because which ids a grid shows depends on its sort and the screen size
    (run 49 tapped a photo above the screen).
60. **2026-10-25: *Sync Settings* in the Select Several strip** (`button:selectSeveralSync`), left
    of *Done*. It runs the existing `develop.sync` with its default groups (the same as Edit › Sync
    Settings: the active photo's settings to the other selected photos), so the touch flow is
    long-press › Select Several › taps › Sync Settings, with no group dialog (Lightroom's Sync
    dialog can come later if Ivan asks). It is greyed out until two photos are selected; the toast
    says "Sync Settings: N photos" from messages every language already has (no new `tr_format!`
    message to translate 13 times).
61. **2026-10-25: Reset a slider by long-press on its name** on iPad: the menu has one item,
    *Reset <name>* (`sliderMenu:reset:<control>`), which does what a double-tap on the name or knob
    does. The desktop's "double-click to reset it" tooltip needs hover, which a finger can't do, and
    a long-press menu is how iPad apps offer such actions. iOS only plus `cfg!(test)`, so the
    upstream unit test `tests_curve::a_sliders_name_menu_resets_it` checks it (in
    the `tests_curve` module, which CI already runs).
62. **2026-10-25: Preset preview by long-press on iPad.** Resting the pointer on a preset previews
    it in the loupe on the Mac; a finger can't hover. On iOS (and in unit tests) a preset's
    long-press menu now shows the preset's look in the loupe for as long as the menu is open, and
    its first item is *Apply* (`presetMenu:apply:<id>`, same as a tap: applies at 100 % with a
    toast). Closing the menu any other way leaves the photo unchanged. Chosen over "preview while
    the finger is down" because a long-press already opens this menu, and a tap must stay "apply".
    Checked by `tests_curve::a_presets_long_press_menu_previews_and_applies_it`.
63. **2026-10-25: Version (snapshot) and History previews by long-press on iPad**, the same pattern
    as presets (62). On iOS (and in unit tests) while a version's long-press menu is open the loupe
    shows that version, and its first item, *Restore*, now has an id
    (`versionMenu:restore:<i>`). A History step's long-press menu previews the photo as it was at
    that step and starts with *Restore* (`historyMenu:restore:<i>`, same as a tap on the step). No
    preview for the version or step the photo is already at. LightCraft's "snapshots" are
    *Versions* (right strip), so "snapshot preview" in PROGRESS meant this panel. Checked by
    `tests_curve::a_versions_long_press_menu_previews_and_restores_it` and
    `tests_curve::a_history_steps_long_press_menu_previews_and_goes_back_to_it`.
64. **2026-10-26: Select Several shows in the loupe too.** While *Select Several* is on, the loupe
    has a pill floating just above the filmstrip's top-left with "Select Several: N photos" and *Done*
    (`filmSelectSeveral`, `button:filmSelectSeveralDone`); before, only the long-press menu's
    *Done Selecting* told you taps in the filmstrip add or remove photos. A floating pill rather
    than a strip like the grid's, so the filmstrip and loupe keep their size. No Sync button there:
    Sync stays in the grid's strip and Edit › Sync Settings. The pill's text is built from existing
    messages (`tr_format!` only accepts messages the translations list; run 66 failed on a new one).
    Checked by `tests_curve::the_loupe_shows_select_several_above_the_filmstrip_and_done_ends_it`
    and the tour shot `10-select-several-loupe`.
65. **2026-10-26: Tour opens panels with `ui.set`, not `panel.*`,** when it can't know whether the
    panel is already open: `panel.edit` and friends toggle, so in run 63 it closed the Edit panel and
    the slider long-press step found no slider. The long-press tour steps are hard checks now.
66. **2026-10-26: The tour checks that the keyboard lifts the UI; the Sync toast lasts 3 s.**
    Run 67 only ever focused fields above the keyboard (search, caption), so the shrink was never
    exercised in CI ("UI shrunk by 0 pt"). The tour now taps a field fully on screen below the
    keyboard's top (Info panel, top 905 to bottom 1150 pt) and expects the new log line "keyboard:
    lifting the UI above the on-screen keyboard"; the keyboard line is logged again whenever the
    shrink changes, not only the height. Both steps are soft until the screenshot
    `04-low-field-keyboard` is checked. The grid strip's "Sync Settings: N photos" toast stays 3 s
    instead of 1.4 s: it reports on photos that may be off screen, and no tour shot (runs 54-67)
    ever caught the shorter one.
67. **2026-10-26: A focused field out of view keeps the keyboard, and scrolls back into view.**
    Run 69: tapping Location in the Info panel lifted the UI, the field then lay below its shrunk
    panel, egui (which reports the box being edited only while it is drawn) reported none, so the
    keyboard dropped, the UI grew back and the keyboard came back, every 150 ms. Two fixes, both
    small: (1) the host keeps the keyboard up while egui still focuses the same text box
    (`keys::hold_field`, unit test `a_focused_field_out_of_view_keeps_the_keyboard`), logging
    "keyboard: focused text field out of view, on-screen keyboard kept up" once; (2) for 30 frames
    after the lift the host sets `text_field::KEEP_IN_VIEW` and a focused `TextField` (and the
    Info panel's GPS box) not fully inside its view calls `scroll_to_me` (upstream patch; the flag
    is never set on the desktop, so only the iPad scrolls). Only for a moment after the lift, so it
    never fights the user scrolling the panel while typing.
68. **2026-11-01: The tour turns the simulator with the Simulator app's Device menu.** `simctl` has
    no rotate command, and RawPad has no reason to rotate itself. CI already drives the Simulator
    with System Events for hardware keys (hard checks since run 37), so the new `hostMenu` tour
    step clicks *Device › Rotate Left* / *Rotate Right* the same way (CI Mac only, never a
    person's computer). The new `waitUntil … "wider"` check reads `ui.inspect` › `window`
    ([820, 1110] pt in portrait on the CI iPad) and waits for it to turn wide and back. Shots
    `12-landscape-loupe`, `12-landscape-grid`, `12-portrait-again`. Branch run 73 turned it and
    back, the shots show the whole UI laid out wide (menu bar, loupe, filmstrip, Edit panel, toolbar
    clear of the home indicator) and portrait again afterwards, so both waits are hard checks. No
    new "safe area" log line after turning (none expected: the insets stay bottom-only), so that
    check was dropped. Split View and Stage Manager can't be scripted in the simulator: they stay
    on Ivan's iPad list.
69. **2026-11-01: The measured keyboard height is right.** Run 72's `04-low-field-keyboard`: the
    keyboard's top is at 1065 of 1400 px, 898 of 1180 pt, so it covers 282 pt, 257 pt above the
    25 pt safe area, exactly what the app logged ("covers 257 pt"). Off the queue.
70. **2026-11-01: The 24 MP memory test uses a real raw that CI downloads, not a committed file.**
    Media is never committed, and the sandbox can't download. CI's new step *Fetch a 24 MP raw*
    downloads the CC0 Sony A7 III compressed ARW from raw.pixls.us (the same file and sha256 as
    `cargo xtask corpus`, `arw-sony-a7m3-compressed.arw`), checks the hash and copies it into the
    app's own `tmp/` folder after install. The tour imports it (`library.import` with
    `$APPDATA/tmp/...`, the new id captured as `$rawId`), opens it, edits, zooms in 4×, and logs
    memory at each point to `memory.jsonl` (`ui.inspect` › `memory` plus the process's resident
    size from `ps`: simulator apps are Mac processes). New `sim_test.py` features: `ifFile` (skip
    with a warning when the download failed, so a raw.pixls.us outage never turns CI red),
    `capture`, `$name` substitution, `memory` steps with `maxRssMB`. The 2500 MB limit is soft
    until a few runs show the normal range (an iPad with 6 GB lets an app use roughly 3 GB). The
    simulator's numbers are a Mac's, not the A16's; they show trends and leaks, not the real limit.
71. **2026-11-01: A memory warning drops the engine's cached photos.** Before, the host only saved
    and returned freed pages. Now `memtrim::trim_caches` squeezes the media cache budget to 16 MB
    for a moment (evicting least recently used decodes and rendered previews; the photo on screen
    is never evicted) and puts the budget back, then frees the GPU's recycled buffers
    (`gpu::trim_pool(0)`) and calls `memory::release`. No upstream patch: `session.media` is
    public. Logs "memory warning: caches held X MB, now Y MB". Unit tests run on the macOS host
    (`lightcraft-engine` is a dev-dependency of `rawpad-ios` now). The tour sends one with the
    Simulator's *Debug › Simulate Memory Warning* (`hostMenu`) and checks the log line (soft
    until seen working). The renderer's per-view stage caches are left alone (private upstream
    API; they already trim to a budget).
72. **2026-11-01: Crash safety is tested by killing the app in the CI tour.** At the end of the
    tour the app rates photo 2, opens it, goes to the background (the tour launches the Settings
    app: `{"background": …}`, winit `suspended` saves and logs "suspended: library and view
    saved"), then `{"relaunch": true}` ends it with `simctl terminate` (no callback, like iPadOS
    ending a suspended app or jetsam) and launches it again with the same environment. The log
    after a relaunch goes to `app-stderr-2.log` (checkLog reads it). Hard checks: the library
    opens again, has at least 24 photos and photo 2 is still rated 4. Soft: the suspend log line,
    a non-zero "log records replayed" and the open photo coming back. Shot `14-relaunched`.
    The memory warning (DECISIONS 71) runs right after the 24 MP raw test, followed by a
    `memory` step (`after-memwarn`), so `memory.jsonl` shows what the trim gives back.
73. **2026-11-01: The relaunch checks and the memory limits are hard.** Run 80 passed every soft
    check of DECISIONS 72 (the suspend line logged, 22 log records replayed, `selection` = [2],
    shot `14-relaunched` shows photo 2 open in the loupe with 4 stars), so they fail the run now,
    plus a new hard `loupe.photo` = 2. The memory steps' `maxRssMB` is hard at 1500 MB (runs
    75–80: at most 522 MB resident) and a new `maxEngineMB` (ui.inspect › memory › engineBytes,
    the engine's own caches; budget 1536 MB on the CI simulator) is hard at 1200 MB (at most
    637 MB so far). The tour ends with a `memory` step in the square grid after the relaunch
    (shot `15-grid-after-relaunch`) as the baseline for P6's thumbnail and preview budgets.
    Raised limits need a new entry, not a quiet edit.
74. **2026-11-01: The iPad keeps at most 3 loupe-sized decodes.** `memory.jsonl` (run 83) showed
    `previewSources` at 4 × 52 MB (≤ 2560 px, linear float; the engine's default capacity is 4).
    `MediaCache::preview_capacity` is public, so the host sets it to 3 when the session opens
    (`memtrim::set_ipad_limits`, no upstream patch), like the web build: both sides of Compare
    plus the next photo still fit. The preview level itself (2560 px) stays: it matches the
    12.9" iPad screen's long edge, and lowering it would need an upstream patch. Unit test on the
    macOS host (5 previews rendered, at most 3 kept); the tour's memory steps get a hard
    `maxPreviews` = 3.
75. **2026-11-01: No extra thumbnail cap on the iPad for now.** Thumbnail sources are capped by
    the engine at 384 MB inside the shared cache budget, are evicted least-recently-used with
    everything else, and are dropped by a memory warning (DECISIONS 71). After a relaunch the
    grid draws from the rendered disk cache: `grid-after-relaunch` (run 83) held 0 thumbnail
    sources (vs 50 MB / 24 at `before-raw`). Revisit if Ivan's real library (hundreds of photos)
    shows memory warnings or ends while scrolling the grid.
76. **2026-11-01: The iPad keeps at most 64 MB of recycled GPU buffers.** Run 85's `memory.jsonl`:
    zooming into the 24 MP raw took GPU allocations from 117 to 512 MB, 177 MB of it the free
    pool (the engine keeps an eighth of its budget, 192 MB on the CI simulator). Leaving the
    zoomed view already gives it all back (`after-raw`: 115 MB, 0 pooled; upstream's idle trim
    frees the pool once no zoom window is shown), so the pool only matters for the zoomed peak.
    `memtrim::set_ipad_limits` settles the process budget, then calls `gpu::set_pool_limit(64 MB)`
    (public API, no upstream patch). Only a budget change from the settings puts it back to an
    eighth. The tour's `raw-zoomed` and `after-raw` steps get `maxGpuPooledMB` = 64, a warning
    until a run shows it working. Open: the resident size rose from 450 MB (zoomed) to 541 MB
    after zooming out in run 85, while the engine's own numbers fell; the memory warning brings
    it back to 265 MB.
77. **2026-11-01: RawPad returns the allocator's free pages to iPadOS itself.** Upstream's
    `lightcraft_sysmem::release_free_memory` calls `malloc_zone_pressure_relief` only on macOS
    (`cfg(target_os = "macos")`), and the iOS host never set the engine's release hook, so
    `memory::release` (after idle GPU trims and memory warnings) did nothing on the iPad. iOS has
    the same libmalloc, so `native::release_free_memory` makes the same one FFI call (in
    `native.rs`, the one module allowed `unsafe`, with a `// SAFETY:` note) and `ios.rs` installs
    it with `memory::set_release_hook` before the session opens. The memory warning logs
    "memory warning: returned X MB of free pages" (tour: soft check). No upstream patch.
    Expected effect: the resident size falls after zooming out of a raw (run 85: 450 → 541 MB).
78. **2026-11-02: The GPU pool check is hard; the page release stays in, unproven.** Branch run 88
    (`p6-gpu-pool`): `raw-zoomed` pooled 54.7 MB (limit 64; run 85: 177 MB) and allocated
    390 MB (run 85: 512 MB); shot `13-raw-zoomed` still draws the zoomed raw. So
    `maxGpuPooledMB` now fails the tour (unless the step says `"soft": true`), like
    `maxPreviews`. The free-page release (DECISIONS 77) logged "returned 0.0 MB" in the
    simulator, and `after-raw` resident size was 564 MB (run 85: 547 MB): no measurable effect
    there (the simulator's malloc may differ from a real iPad's). It is harmless and stays; its
    tour check stays soft. Merged into main the same night.
79. **2026-11-02: Brush and Remove strokes start where the finger touched.** egui reports a drag
    only after the pointer moved past its threshold, so the brush and spot tools began the stroke
    there and lost its first few millimetres (editing audit, item 15; the crop and targeted
    adjustment already use `press_origin()`, DECISIONS of the touch-target work). On iOS and in
    unit tests, `detail.rs › stroke_lead_in` puts the press point in front of the first sample
    when a new stroke starts with a drag (clicks and still holds are unchanged; pressure is
    copied to the extra point). Desktop behaviour is untouched. Unit test in `tests_curve.rs`.
80. **2026-11-02: Colour labels in the bottom bar on the iPad.** Labels were reachable only
    through the Info panel, the Photo menu or the 6–9 keys (editing audit, item 30). On iOS (and
    in unit tests) the rating pill ends with a 22-point label button (`icon:labelMenu`): a dot in
    the photo's label colour, a ring when it has none. It opens the same label menu as the grid's
    context menu (ids `bottomLabel:<colour>`, `bottomLabel:none`) and runs `photo.label` with the
    Compare/Survey target like the stars. The pill grows 28 points; when the bar is too narrow
    the button goes first (before the pill itself), so the stars and flags never disappear because
    of it; Copy/Paste Settings may hide 28 points sooner (they are in the Edit menu too). Desktop
    unchanged. Unit test in `tests_curve.rs`, tour shots `08-label-menu` and `08-label-green`.
81. **2026-11-02: A compact rating pill on a narrow iPad bottom bar.** In portrait (820 pt) the
    bottom bar's room between the view modes and the zoom group is about 205 pt, less than the
    full pill (208 pt, 236 with the label button), so the iPad showed no stars, flags or label
    there at all (CI run 93). On iOS (and in unit tests), when the full pill doesn't fit but 98 pt
    do, a compact pill stands in: a 50-point rating button (`icon:ratingMenu`: a star, the
    count, and the pick/reject flag if set) whose menu holds the same stars and flags
    (`star:bottom:<n>`, `flag:<id>`), then the label button. The stars/flags code moved to one
    helper (`rate_and_flag`) shared by both pills. Desktop unchanged (the pill still just hides).
    Unit test `a_narrow_bottom_bar_shows_the_compact_rating_pill`; tour shots `08-compact-pill`,
    `08-rating-menu`, `08-rated-4`.
82. **2026-11-02: The last run of the night doesn't merge code.** A main run takes about 40
    minutes with the release, longer than what is left of a session after a branch run (25–35
    minutes). The last run of the night (02:54 Berlin) must leave main green, so it leaves a
    green branch unmerged and the next session merges it at its start and watches the main run.

## 2026-11-08: Upstream 0.19.0 (scheduled session, 18:56 Berlin)

83. **2026-11-08: Merge upstream v0.19.0 now, in a branch, with v0.9.1 as the explicit base.**
    Earlier checks reported "newest tag still v0.9.1" because `sort` put `v0.9.1` after
    `v0.19.0` as text; upstream had released 0.10 through 0.19 (latest 2026-11-06). The fork
    was a snapshot without upstream history (no merge base), so `git merge` can't work: the
    branch `upstream-0.19.0` merges with `git merge-recursive v0.9.1 -- HEAD v0.19.0` and
    records v0.19.0 as the second parent, so future upstream merges have a normal merge base.
    Ten releases at once breaks the weekly policy, but merging them one at a time would mean ten
    rounds of the same conflicts. Upstream moved most patched files (`panels/dialogs.rs` →
    `dialogs.rs`, `panels/detail.rs` → `panels/detail/*.rs`, `panels/right.rs` → `right/*.rs`,
    `panels/edit.rs` → `edit/*.rs`, `text_field.rs` → `widgets/text_field.rs`, stars →
    `widgets/rating.rs`, `menus.rs` → `menus/files.rs`, `export.rs` → `export/params.rs`):
    upstream's version of each conflicting file was taken and every iPad patch re-applied by
    hand (old patches saved in `ios/upstream-patches-v0.9.1.diff`). egui moves to 0.37 on
    storytold's fork (`Id::new` is `Id::unique` there); the host's `egui-winit`/`egui-wgpu`
    follow. The upstream workflows (old and new: `ci.yml`, `ci-windows.yml`, `house-rules.yml`,
    `sonarcloud.yml`) stay deleted, as in DECISIONS 10. Main gets the merge only after a green
    branch run with a clean tour.
84. **2026-11-08: iPad UI tests live in `crates/ui-egui/src/tests/ipad.rs`.** Upstream moved
    `tests_curve.rs` to `tests/curve.rs`; our tests (and copies of its five small helpers) now
    have a file of their own that upstream never touches. CI runs `--lib tests::ipad`.
85. **2026-11-08: Check upstream tags with `sort -V`.** `git ls-remote --tags
    https://github.com/storytold/lightcraft | grep -v '\^{}' | awk '{print $2}' | sort -V | tail -3`.
86. **2026-11-08 (20:00 Berlin): No upstream self-update check on the iPad.** Upstream 0.19
    added craft-update (download a newer desktop build). RawPad is installed and updated through
    SideStore, so on iOS the Help › Check for Updates item is hidden and the command answers with
    a SideStore hint; the startup check stays off (the iOS host never turns it on).

## 2026-11-08: Upstream 0.19.0 tour (scheduled session, 20:55 Berlin)

87. **2026-11-08: No LightCraft startup dialogs on the iPad.** Run 99's 24 failed tour taps all
    had one cause: upstream 0.19's first-run *Welcome to LightCraft* walkthrough (a modal) sat
    over the app, so every press was refused (`08-before-after-menu.jpg`). Its pages and *What's
    New* describe the desktop (keyboard shortcuts, File menu), and RawPad's release notes are in
    SideStore, so `startup::open_at_startup` returns nothing on iOS.
88. **2026-11-08: The memtrim tests settle the process memory budget once before they start.**
    Run 100 failed `the_ipad_pools_at_most_64_mb_of_gpu_buffers` (165 MB pooled) though run 99
    passed it: the budget is computed lazily by the first thread that asks, and computing it
    resets the GPU pool limit to an eighth of it, so a parallel test's late first `budget()`
    undid the iPad limit. A `Once` in the test module makes every test wait for one settled
    budget. On the iPad the host sets the limits once at launch, before any render.


## 2026-11-08: Safe-area insets (scheduled session, 21:55 Berlin)

89. **2026-11-08: The host gives egui zero safe-area insets once it has inset the screen itself.**
    Runs 99–102 refused every press on the bottom bar (Before/After, its ⌄ menu, the compact
    rating pill, the label button) as "no longer visible", though headless tests passed. Cause:
    the host shrinks `RawInput::screen_rect` to UIKit's safe area (since run 8), and egui-winit
    0.37 now also reports those insets, which egui keeps. egui's background layer is
    `viewport_rect - safe_area_insets`, so the insets came off twice and the layer ended about
    20 pt above the screen's bottom: `layer_id_at` found no layer under the bar (also for real
    touches, probably). The host now sends `safe_area_insets = 0` with its inset rect; the
    rect stays our own (it also carries the keyboard lift and the minimum top inset).

## 2026-11-08: Reset by long-press (scheduled session, 22:55 Berlin)

90. **2026-11-08: The curve channel buttons and the colour wheels get long-press reset menus.**
    Both reset on a double-click that only a hover tooltip mentions (audit item 6, "the reset
    hints"); a finger never sees the tooltip. As with the slider names (61): on iOS (and in unit
    tests) a long-press on a curve channel button opens the curve's existing reset menu for that
    channel (`curveMenu:resetChannel`, `curveMenu:resetAll`); a long-press on a colour wheel opens
    "Reset <wheel>" (`wheelMenu:reset:<key>`). The double-tap still works. Tests in `tests/ipad.rs`.

## 2026-11-08: Hover-only hints, the rest (scheduled session, 23:54 Berlin)

91. **2026-11-08: Picking several mask effect presets by touch = the row's long-press menu.**
    Went through every remaining `double_clicked()` and hover hint in the UI crate. Crop's
    straighten hint ("double-click for Auto") already has an *Auto* button beside it; History's
    versions ("double-click to rename") already have *Rename…* in their long-press menu; masks,
    mask components and keywords have long-press menus with the same actions; the histogram's
    double-tap reset works with a finger. The one real gap: Manage Effect Presets picks several
    presets only with ⌘ or Shift. On iOS (and in unit tests) a long-press on a row now opens
    *Add to Selection* / *Remove from Selection* (`effectMenu:toggle:<name>`); a tap still
    selects just that row, and a group heading still selects the group. No new mode or strip
    (unlike the grid's Select Several, 58): the window is small and rarely used.

94. **2026-11-09: `lightcraft-sysmem` is dropped from `apps/rawpad-ios/Cargo.toml`** (scheduled
    session, 01:54 Berlin). Nothing on iOS uses it since DECISIONS 77 (our own
    `native::release_free_memory`); neither shared desktop file (`control_server.rs`,
    `logging.rs`) mentions it. The earlier worry that Cargo.lock can't be regenerated here was
    wrong: the committed Cargo.lock has no `rawpad-ios` entry at all (CI adds it on every build,
    without `--locked`), and `lightcraft-sysmem` stays in it for the desktop app, so the lock
    needs no edit.
