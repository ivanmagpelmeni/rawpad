# Progress

## Latest session summary

**2026-10-19 (nightly 9, 02:55–03:40 Berlin, unattended, last run of the night)**
1. Main run 35 green, **released 1.35** (hardware keyboard read from UIKit).
2. Its log showed CI's `5` arriving as the *keypad* 5, which RawPad ignored (so the rating probe proved nothing): keypad digits now count as number keys.
3. **On-screen keyboard:** tapping a text field (search box, keywords…) now brings up the iPad keyboard (`native::TextKeyView`, DECISIONS.md 43); screenshot `runs/37/04-text-field` shows it under the focused search box. It goes away when the field loses focus.
4. Branch run 37 all green; for the first time CI proved hardware keys end to end (`5` → 5 stars, ⌘Z → back to 3). Merged into main; main run 38 passed build, clippy and the tour by 03:38 and should release 1.38.
5. Upstream not checked (not the first run of the night).

**2026-10-19 (nightly 8, 01:54–02:40 Berlin, unattended)**
1. Main run 32 green, released **1.32** (Pencil + ⌘ keys); `03-brush` screenshot checked (Mask 2, exposure −1).
2. Found that winit passes **no hardware keys** on iOS (run 30's probe even ran under the share sheet).
3. **Hardware keyboard now works in the simulator:** RawPad reads key presses from UIKit itself (`native::KeyView`). Branch run 33: `p`, ⌘ and Z all reached the app (`keyboard (UIKit):` log lines). Fixed a stuck-⌘ bug seen in that log.
4. CI tour now presses `5` (rating) and ⌘Z (undo) and checks the photo's stars (soft check for now).
5. Branch `p4-uikit-keys` merged into main at handoff (its second run compiled and passed clippy); main run 35 should release it (about 1.35).

**2026-10-19 (nightly 7, 00:54–01:40 Berlin, unattended)**
1. Main was **red** (run 26: the simulator's Photos never answered). Fixed the CI tour (Photos step only warns, Share check no longer needs Photos); main run 27 green, released **1.27** (pinch/pan, top gap, translated Share label).
2. **Apple Pencil:** a resting Pencil tip now starts a brush stroke at once instead of opening the long-press menu. Branch run 28 green; merged into main (the next main run should release it, about 1.31).
3. CI tour now paints a **brush mask** (screenshot `03-brush` shows "Mask 2", the brush panel, exposure −1).
4. **Hardware keyboard:** ⌘ becomes the shortcut key as on the Mac, plus a CI keyboard probe; branch run 30's tour green, merged into main at the end (its main run is the first CI of both together).
5. Upstream not checked (not the first run of the night).

**2026-10-19 (nightly 6, 23:55–00:40 Berlin, unattended)**
1. **Pinch-zoom and pan in the loupe** are now in the CI tour (zoom to 45 %, two-finger and one-finger pan; screenshots `04-pinch-zoom`, `04-panned`).
2. **Top gap of 20 pt** so the iPad's multitasking "•••" no longer covers the search box.
3. **Share Exported Photo…** button now speaks the app's language (14 languages).
4. CI flakiness found and fixed: a cold simulator took 90 s+ for Add to Photos; CI now warms Photos up first.
5. All merged into main at handoff (branch run 25's tour green); the main run should release it (about 1.26).

**2026-10-18 (nightly 5, 22:54–23:35 Berlin, unattended)**
1. **Fixed the Photos hang:** Export dialog + *Add to Apple Photos* now finishes (async Photos call with a timeout; run 20 saved the photo in under a second).
2. **Share Exported Photo…** button after an export: run 20 green, screenshots show the button and the share sheet for the new JPEG.
3. Branch `p5-share-export` merged into main; main run 21 green, released **1.21**.
4. Upstream not checked (not the first run of the night).
5. Next: a pinch-zoom step in the tour, then the top inset under the multitasking "•••".

**2026-10-18 (nightly 4, 21:55–22:45 Berlin, unattended)**
1. Run 16 checked: green, released **1.16** (share sheet). Main untouched this session except docs.
2. Built **Share Exported Photo…** (a button after an export → share sheet for the new files) on branch `p5-share-export`; unit tests green.
3. The tour now waits for results (`waitUntil`) instead of fixed sleeps.
4. **Found a bug:** an Export-dialog export with *Add to Apple Photos* hangs at "Adding to Apple Photos…" in the simulator (run 19). Probably in 1.14/1.16 too. Fix first next session.
5. Branch not merged (run 19 red because of that hang). See HANDOFF.md.

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
- [~] **4. Input:** done: long-press → context menu, safe area from UIKit, top gap for the multitasking button, pinch-zoom/pan in the loupe (tested in CI with egui events), Pencil skips the long-press wait (brush strokes; DECISIONS.md 39), brush stroke in the CI tour, hardware keyboard read from UIKit with ⌘ as command (DECISIONS.md 40, 41; keys reach the app in CI run 33). To do: real-finger and Pencil check on the iPad; rotation, Split View / Stage Manager resizing
- [~] **5. Files:**
  - Done: Files picker import (photos and folders); JPEG export to `Documents/Exports`;
    Export › Add to Apple Photos (`native::photos_runner`).
    Share sheet (`native::share`, behind every *Show in Finder*), which also saves originals to Files.
    **Share Exported Photo…** button right after an export (`src/share_offer.rs`).
    Export dialog + Add to Apple Photos no longer hangs (DECISIONS.md 33).
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

Main run 35 green, released **1.35** (hardware keyboard from UIKit). Branch `p4-soft-keyboard`
(on-screen keyboard for text fields, keypad digits; run 37 all green, keyboard screenshot checked)
was merged into main at the end of nightly 9; the main run after that push should release it
(about 1.38). CI checks build, fmt, unit tests, clippy, the simulator tour (sliders, radial and
brush masks, pinch/pan, keyboard probe with rating 5 and ⌘Z undo, search box with on-screen
keyboard, background export, Share button and sheet, Add to Apple Photos as a soft check), the
export file and the log file.

## Next 3 tasks

1. Check the main run after the `p4-soft-keyboard` merge (green, released ~1.38?). If both
   keyboard `waitUntil rating` steps say `ok` again, drop `"soft"` from them (run 37 was the first
   time they did).
2. Keyboard avoidance: the on-screen keyboard covers the bottom ~40 % of the screen and egui
   doesn't know. Read the keyboard frame (`UIKeyboardWillChangeFrameNotification` or
   `keyboardLayoutGuide`) and shrink the egui screen rect so a focused field near the bottom
   (keywords, captions) stays visible. Also type into the search box in CI (`hostKeys`) and
   check the text arrives once.
3. Rotation and Split View / Stage Manager resizing in the tour (`simctl` can't rotate: try the
   Simulator menu via osascript on the CI Mac).

Also queued:
- A 24 MP raw memory test in the simulator (CC0 ARW/RW2 from `cargo xtask corpus`).
- Delete merged branches (no tool for it from the sandbox).

## Needs Ivan

**This week, in short:** install SideStore (steps 1–2 below) if it isn't there yet, update RawPad
to the newest version (1.38 if it's listed, else 1.35), then try in this order and tell Claude in
chat what you see: open the app → import a few photos → open one, move sliders → pinch to zoom →
Pencil brush mask → export with *Add to Apple Photos* → *Share Exported Photo…* → tap **Search
Photos** (on-screen keyboard) → with a hardware keyboard, **5** and **⌘Z**. Details for each are
in step 3.

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
   - **Fixed in the newest version (1.21):** in the Export window tick **Add to Apple Photos**,
     then Export. (Older versions could get stuck at "Adding to Apple Photos…".) The first time,
     iPadOS asks whether RawPad may add photos: tap **Allow**. Is the photo in the Photos app?
   - **New (1.21):** right after an export a **Share Exported Photo…** button appears at
     the bottom of the photo for 20 seconds. Tap it: does the share sheet open with your photo?
   - **New (1.16):** press and hold a photo you imported → **Share…**: the iPad's share sheet
     opens (Save to Files, AirDrop, Mail). **Menu › Help › Open Log Folder** now shares the log file
     directly.
   - **New (1.12):** if something goes wrong, open **Files › On My iPad › RawPad › Logs**, press
     and hold **lightcraft.log** › **Share**, and send it to Claude in chat (the previous run's
     log is **lightcraft.1.log**).
   - **New (1.27):** open a photo and pinch with two fingers: does it zoom in? Drag
     with one or two fingers: does it move around? The **•••** button at the top middle of the
     screen should no longer sit on the search box.
   - **New (1.31 or later), Apple Pencil:** open a photo › masking (the dotted circle icon on
     the right) › **Brush**, and paint with the Pencil. Does a stroke appear right away, even if
     you rest the tip first? If strokes stay invisible, untick **Pen Pressure** in the brush
     settings and try again (the USB-C Pencil has no pressure), and tell Claude.
   - **New (1.38 or later), on-screen keyboard:** without a keyboard attached, tap the
     **Search Photos** box at the top: the iPad keyboard should come up. Type a word: does it
     appear in the box, and does the delete key work? Tap somewhere on the photo: does the
     keyboard go away? (Also try a keyword field in the right panel's info section.)
   - **New (1.35 or later), keyboard:** if you have a keyboard for the iPad, open a photo and
     press **5** (five stars?), then **⌘Z** (undo: back to the old stars?). Also try ⌘Z after a
     slider change. Tell Claude what happens; the log (see above) shows `keyboard (UIKit):` lines.
   - Anything slow, frozen or crashing: say which photo type (ARW or RW2) and what you did.
