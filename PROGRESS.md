# Progress

## Latest session summary

**2026-10-18 (nightly 1, 18:54–19:40 Berlin, unattended)**
1. No new upstream release (still 0.9.1). Main was green (run 8).
2. Long-press (500 ms, still finger) now opens context menus: grid, folders, masks (`src/touch.rs`, unit-tested in CI).
3. The UI now keeps out of the home indicator (safe area read from UIKit; egui-winit's was empty).
4. Interface settings (panels, language, appearance) are kept between launches (`RawPad UI.json`).
5. The AI mask tiles are hidden on iPad. See HANDOFF.md for CI status of these commits.

**2026-10-18 (kickoff, interactive, ended about 15:10)**
**2026-10-18 (kickoff, interactive, ended about 15:10)**
1. Kickoff finished: brief, fork scaffolding, repo-memory docs, SideStore steps, and the Sunday
   session schedule (18:54–02:54 Berlin, hourly).
2. The iOS host (winit + egui-wgpu on Metal) compiles, and CI runs green on a standard macOS
   runner.
3. Simulator tour passes: library grid, Develop, sliders, radial mask, JPEG export to
   `Documents/Exports`. Screenshots are on the `ci-artifacts` branch.
4. iPad features so far:
   - Files picker for imports, with copies into the library.
   - Folders (SD card) opened in place.
   - Safe area; status bar hidden.
5. **RawPad 1.8 is released** and listed in the SideStore source. Not yet installed on a real
   iPad: that's phase 3 and needs Ivan.

## Phases

- [x] **0. Kickoff:** brief, public fork, docs, CI, SideStore steps (`SETUP.md`)
- [x] **1. Engine compiles for iOS** (sim + device) in CI. (A pixel comparison against the CPU
      reference is still to add to the tour.)
- [x] **2. App shell** launches on the iPad simulator with the LightCraft UI on Metal and the demo
      library; CI screenshots
- [ ] **3. First install on Ivan's iPad via SideStore**: release 1.8 is ready, waiting for Ivan
- [~] **4. Input:** done: long-press → context menu, safe area from UIKit. To do: touch → pointer mapping, pinch-zoom/pan in loupe, Pencil (USB-C, no pressure)
      for brush masks, hardware-keyboard shortcuts; rotation, Split View / Stage Manager resizing,
      safe areas
- [~] **5. Files:**
  - Done: Files picker import (photos and folders); JPEG export to `Documents/Exports`.
  - To do: save to the Photos app, the share sheet, original-raw export to Files.
- [ ] **6. Performance and memory on the A16 (~6 GB):**
  - tiling, preview downscaling, a smaller thumbnail cache
  - memory warnings
  - backgrounding and crash safety
  - 24 MP ARW/RW2 tests
- [ ] **7. Parity sweep against upstream** (minus AI and the second display):
  - touch polish
  - ~~hide the AI mask tiles~~ (done); still: AI items in mask context menus, and the
    ArtCraft Discord/website links in the iPad build
  - craft-fonts in release builds
  - routine upstream merges

## Current status

Main is green: run 9 published release 1.9 (long-press). Run 10 (safe area, prefs, AI tiles) was running at handoff. CI checks build, fmt, unit tests, clippy, the simulator tour,
and that an export file is written.

## Next 3 tasks

1. **Check the screenshots of the safe-area/prefs/AI-tiles build** (see HANDOFF.md): the bottom
   toolbar must sit above the home indicator, the mask panel must show 5 tiles (Brush, Linear,
   Radial, Luminance, Color), and `app-stderr.log` must show a `safe area (points): …` line.
   Then add a tour step that sends `ui.zoom` and screenshot pinch-zoom state in the loupe.
2. **Save to the Photos app and the share sheet** after export: `NSPhotoLibraryAddUsageDescription`
   in `ios/Info.plist`, `UIImageWriteToSavedPhotosAlbum` (or `PHPhotoLibrary`) and
   `UIActivityViewController` in `src/native.rs`. Also "export original raw" to Files.
3. **On-device log file** in `Library/Caches/rawpad.log` (the logger in `ios.rs` writes stderr
   only), so Ivan can share a log after a crash. Then hide the remaining AI items (mask context
   menus) and the ArtCraft links behind `cfg!(target_os = "ios")`, logged in `UPSTREAM.md`.

Also queued:
- A 24 MP raw memory test in the simulator (CC0 ARW/RW2 from `cargo xtask corpus`).
- Pencil: brush masks with the Apple Pencil (USB-C, no pressure); hardware-keyboard shortcuts.

## Needs Ivan

1. **Install SideStore** once from your Windows PC: `SETUP.md` parts A–D.
2. **Install the newest RawPad** (1.8 or later; SideStore shows the newest): `SETUP.md` part E
   (add the source, then tap Free/Get). Already installed? SideStore › My Apps › tap **Update**.
3. **Test and tell Claude in chat what you see.** Try:
   - Does it open? Does the interface fit the screen in landscape and portrait?
   - Tap **+** in the top bar (Import Photos…), pick a few photos from Files or the SD card,
     then confirm **Import**. Do they appear?
   - For a whole SD-card folder: **Menu › File › Import from Folder…**.
   - Open one, move some sliders, try a radial mask.
   - **New:** press and hold a photo in the grid (or a folder, or a mask) for half a second:
     a menu should open. Does it?
   - Export it: **Export** → **Export**. It lands in **Files › On My iPad › RawPad › Exports**.
   - Anything slow, frozen or crashing: say which photo type (ARW or RW2) and what you did.
