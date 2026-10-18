# Progress

## Latest session summary

**2026-10-18 (nightly 3, 20:55–21:35 Berlin, unattended)**
1. Run 14 checked: green, released as 1.14 (Photos export). No new upstream release check needed (not the first run).
2. **Share sheet** on iPad: every *Show in Finder* is now **Share…** (original raw, folder, Help › Open Log Folder → the log). *Save to Files* in the sheet covers "export original raw".
3. Tested on branch `p5-share` (run 15): tour green, screenshot `05-share-sheet` shows the sheet with *Save to Files*. Merged into main at handoff (run 16 should release it).
4. 24 MP GPU fallback judged simulator-only (its Metal reports 256 MiB buffers); verify with a device log (DECISIONS.md 30).
5. Open: the Photos import still reads `running` 15 s after export in the simulator.

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
    Share sheet (`native::share`, behind every *Show in Finder*), which also saves originals to Files.
  - To do: a Share button right after an export (the exported file isn't in the library).
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

Main is green: run 14 released **1.14** (Add to Apple Photos). The share sheet was tested green on
branch `p5-share` (run 15) and merged into main at handoff (run 16 pending, should release 1.16).
CI checks build, fmt, unit tests, clippy, the simulator tour (incl. a share-sheet screenshot),
the export file, the log file, and (in replies.jsonl) the Photos import.

## Next 3 tasks

1. **Check run 16** (share merge): green, released, `runs/16/05-share-sheet.jpg` shows the sheet.
2. **Share right after export:** the export dialog / finished-export toast has no button for the
   exported file. Find where upstream reports a finished export (`Session::last_export`, export
   toasts in `crates/ui-egui`) and add a host-side or tiny `cfg(ios)` "Share…" that calls
   `services.reveal` with the exported path. Test on a branch first (DECISIONS.md 28).
3. **Photos import still `running`** after 15 s in the tour (runs 14, 15): poll
   `export.photosImports` until `running` is false (up to ~60 s) and fail the tour if it never
   says `"imported": 1`. Then a `ui.zoom` pinch step in the tour.

Also queued:
- Top inset: iPadOS's multitasking "•••" sits over the search box (status bar hidden, top inset 0).
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
   - **New (1.16):** press and hold a photo you imported → **Share…**: the iPad's share sheet
     opens (Save to Files, AirDrop, Mail). **Menu › Help › Open Log Folder** now shares the log file
     directly.
   - **New (1.12):** if something goes wrong, open **Files › On My iPad › RawPad › Logs**, press
     and hold **lightcraft.log** › **Share**, and send it to Claude in chat (the previous run's
     log is **lightcraft.1.log**).
   - Anything slow, frozen or crashing: say which photo type (ARW or RW2) and what you did.
