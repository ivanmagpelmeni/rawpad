# Progress

## Latest session summary

**2026-10-18 (nightly 2, 19:54–20:40 Berlin, unattended)**
1. Run 10 checked: toolbar clears the home indicator, mask panel shows 5 tiles. No new upstream release.
2. On-device log: `Files › On My iPad › RawPad › Logs › lightcraft.log` (panics recorded). Released in **1.12**.
3. AI items hidden in the mask Add/Subtract/Intersect menus too (1.12).
4. **Export › Add to Apple Photos** works on iPad (tested in the simulator: run 13 saved a photo to Photos). Merged to main at handoff; run 14 should release it as 1.13.
5. Found: 24 MP exports fall back to the CPU (GPU buffer 275 MiB > 256 MiB limit), a phase 6 item.

**2026-10-18 (nightly 1, 18:54–19:40 Berlin, unattended)**
1. No new upstream release (still 0.9.1). Main was green (run 8).
2. Long-press (500 ms, still finger) now opens context menus: grid, folders, masks (`src/touch.rs`, unit-tested in CI).
3. The UI now keeps out of the home indicator (safe area read from UIKit; egui-winit's was empty).
4. Interface settings (panels, language, appearance) are kept between launches (`RawPad UI.json`).
5. The AI mask tiles are hidden on iPad. See HANDOFF.md for CI status of these commits.

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
  - Done: Files picker import (photos and folders); JPEG export to `Documents/Exports`;
    Export › Add to Apple Photos (`native::photos_runner`).
  - To do: the share sheet, original-raw export to Files.
- [~] **6. Performance and memory on the A16 (~6 GB):** (done: on-device log file with panics)
  - tiling, preview downscaling, a smaller thumbnail cache
  - memory warnings
  - backgrounding and crash safety
  - 24 MP ARW/RW2 tests
- [ ] **7. Parity sweep against upstream** (minus AI and the second display):
  - touch polish
  - ~~hide the AI mask tiles~~ and ~~AI items in mask menus~~ (done); ArtCraft links kept
    (DECISIONS.md 26)
  - craft-fonts in release builds
  - routine upstream merges

## Current status

Main is green: run 12 released **1.12** (log file, AI menu items hidden). The Photos export was
tested green on the `p5-photos` branch (run 13) and merged into main at handoff (run 14 pending).
CI checks build, fmt, unit tests, clippy, the simulator tour, the export file, the log file,
and (in replies.jsonl) the Photos import.

## Next 3 tasks

1. **Check run 14** (Photos merge): green, release 1.13, and in `runs/14/replies.jsonl` the
   `export.photosImports` reply shows `"imported": 1`.
2. **Share sheet** after export (`UIActivityViewController` in `src/native.rs`, presented from
   the root view controller, with a popover anchor on iPad or it crashes), and "export
   original raw" to Files. Test on a branch first (DECISIONS.md 28).
3. **24 MP GPU export limit:** exports of 6000×4000 fall back to the CPU because egui-wgpu's
   device asks for `adapter.limits()` and the render needs 275 MiB > 256 MiB
   `max_storage_buffer_binding_size`. Check what the engine's own GPU device requests on iOS
   and whether tiling is possible (phase 6). Then a `ui.zoom` pinch step in the tour.

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
   - **New (1.13):** in the Export window tick **Add to Apple Photos**, then Export. The first
     time, iPadOS asks whether RawPad may add photos: tap **Allow**. Is the photo in the Photos app?
   - **New (1.12):** if something goes wrong, open **Files › On My iPad › RawPad › Logs**, press
     and hold **lightcraft.log** › **Share**, and send it to Claude in chat (the previous run's
     log is **lightcraft.1.log**).
   - Anything slow, frozen or crashing: say which photo type (ARW or RW2) and what you did.
