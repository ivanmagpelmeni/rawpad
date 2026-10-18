# Progress

## Latest session summary

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
- [ ] **4. Input:** touch → pointer mapping, pinch-zoom/pan in loupe, Pencil (USB-C, no pressure)
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
  - hide the AI mask buttons (Object, Describe, Subject, Sky, Background) and the ArtCraft
    Discord/website links in the iPad build
  - craft-fonts in release builds
  - routine upstream merges

## Current status

Main is green: run 8 published release 1.8. CI checks build, fmt, clippy, the simulator tour,
and that an export file is written.

## Next 3 tasks

1. **Long-press → secondary click.** On a touchscreen there is no right-click, so context menus
   (grid, folders, masks) are unreachable. Map long-press to a secondary click in the host, before
   `raw_input_hook`. Also check pinch-zoom and two-finger pan in the loupe, and add a tour step
   that sends a `ui.zoom`.
2. **Save to the Photos app and the share sheet** after export. This needs
   `NSPhotoLibraryAddUsageDescription` in `ios/Info.plist`, `PHPhotoLibrary` (or
   `UIImageWriteToSavedPhotosAlbum`) and `UIActivityViewController` in `src/native.rs`. Also add
   "export original raw" to Files.
3. **Persist UI preferences** (`app.ui`: panels, language, appearance) like the desktop's
   `PrefsWriter`, under `Library/Application Support/RawPad Library/../prefs.json`. Also keep a
   log file in `Library/Caches/rawpad.log` for on-device debugging.

Also queued:
- The bottom safe area: the toolbar sits close to the home indicator; check with a screenshot.
- A 24 MP raw memory test in the simulator (CC0 ARW/RW2 from `cargo xtask corpus`).
- Hiding AI and ArtCraft items behind `cfg(target_os = "ios")`. Log each upstream patch in
  `UPSTREAM.md`.

## Needs Ivan

1. **Install SideStore** once from your Windows PC: `SETUP.md` parts A–D.
2. **Install RawPad 1.8:** `SETUP.md` part E (add the source, then tap Free/Get).
3. **Test and tell Claude in chat what you see.** Try:
   - Does it open? Does the interface fit the screen in landscape and portrait?
   - Tap **+** in the top bar (Import Photos…), pick a few photos from Files or the SD card,
     then confirm **Import**. Do they appear?
   - For a whole SD-card folder: **Menu › File › Import from Folder…**.
   - Open one, move some sliders, try a radial mask.
   - Export it: **Export** → **Export**. It lands in **Files › On My iPad › RawPad › Exports**.
   - Anything slow, frozen or crashing: say which photo type (ARW or RW2) and what you did.
