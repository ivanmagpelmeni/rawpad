# Progress

## Editing audit (2026-10-19)

**Status (2026-10-20, about 11:00 Berlin):** uploaded on the 20th (the audit session couldn't
push), merged with that morning's keyboard avoidance, fixed twice in CI (runs 41 and 42: a tapped
text box no longer brought the keyboard up) and **all green in branch run 43**, then merged into
main. The main run after that merge releases it: **update RawPad in AltStore to the first version
after 1.40** (about 1.44).

How each item was checked: "CI" = seen in the simulator screenshots of run 38; "code" = by reading
the program (no screenshot yet). Statuses are for 1.38, the version on Ivan's iPad.

| # | Item | Status on 1.38 | Note |
|---|---|---|---|
| 1 | Open a photo in Develop, back to the grid | Works (CI) | Double-tap a photo; Back button top left or the grid icon bottom left. |
| 2 | Basic panel sliders (temp, tint, exposure … saturation) | **Broken** (Ivan) | Moving a slider brings up the keyboard. **Fixed in the audit.** |
| 3 | Scrolling the Edit panel with a finger | **Broken** (code) | Starting the scroll on a slider changes that slider. **Fixed in the audit:** an up/down drag scrolls. |
| 4 | White-balance eyedropper, Auto button | Works (code) | Tap the eyedropper, then tap the photo. |
| 5 | Reset one slider (double-tap its name or knob), reset all (Reset button) | Works, hard to discover (code) | The "double-tap to reset" hint only appears on hover, which a finger can't do. |
| 6 | Type a value into a slider | Works (code) | Tap the number. After the fix this is the one slider action that brings up the keyboard. |
| 7 | Tone curve: drag and add points | Works, fiddly (code) | Points are small (10 pt) for a finger. |
| 8 | Tone curve: remove a point | **Unusable by touch** (code) | Needs a precise double-tap on the tiny point. |
| 9 | HSL / colour mixer, targeted adjustment (drag on photo) | Works (code) | The sample spot is taken a little after where the finger lands. |
| 10 | Colour grading wheels | Works (code) | The small "hide" eye on each wheel only shows on hover. |
| 11 | Detail (sharpening, noise), lens corrections, transform / Upright | Works (code) | |
| 12 | Crop, straighten, aspect, rotate, flip | Works, fiddly (code) | Corner handles are small and grabbed a bit off; to finish, double-tap inside the crop (no Done button). |
| 13 | Masks: create brush / linear / radial | Works (CI) | Screenshots 03-masking and 03-brush. |
| 14 | Masks: move, resize, rotate, erase, delete, invert, overlay | Works, fiddly (code) | Pins were invisible until touched (**fixed in the audit**); handles are small, the radial rotate knob is tiny. Erase = the Erase button. |
| 15 | Spot removal / healing | Works (code) | The first few millimetres of a stroke are lost. |
| 16 | Presets: apply, create | Works (code) | |
| 17 | Presets: preview on hover | **Unusable by touch** (code) | Needs hover. The preset thumbnails still show each look. |
| 18 | Copy / paste settings | Works (code) | Bottom-bar Copy and Paste. |
| 19 | Sync settings to other photos | **Unusable by touch** (code) | Picking several photos needs Shift or ⌘; only Select All works by touch. |
| 20 | Undo / redo without a keyboard | **Unusable** (code) | Only Menu › Edit › Undo (3 taps). **Fixed in the audit:** two-finger tap = undo, three-finger tap = redo. |
| 21 | History and snapshots | Works (code) | Tap a history step to go back. The snapshot preview on hover is unreachable. |
| 22 | Virtual copies | Works (code) | Photo menu, or press and hold a photo. |
| 23 | Before/After side by side | Works (code) | The Before/After button in the bottom bar. |
| 24 | Before/After split, top/bottom, split top/bottom | **Broken** (Ivan) | Only via Menu › View, otherwise keyboard shortcuts. **Fixed in the audit:** a small ⌄ button next to Before/After lists every view. Ivan's exact problem isn't confirmed yet: please re-test. |
| 25 | Show original | Hard to reach (code) | Was hold `\` or Menu › View. **Fixed in the audit:** in the same ⌄ menu. |
| 26 | Draggable split divider | Not in the desktop app either | The split line stays in the middle. |
| 27 | Zoom (fit, 1:1, pinch), navigator | Works (CI) | Screenshots 04-pinch-zoom, 04-panned. Real fingers not checked. |
| 28 | Histogram and clipping warnings | Works (code) | Tap the triangles on the histogram. |
| 29 | Ratings, flags | Works (CI) | Stars in the bottom bar are small (19 pt). |
| 30 | Colour labels | Works (code) | Info panel or Photo menu; not in the bottom bar. |
| 31 | Export | Works (CI) | Export, Share button, Add to Apple Photos. |
| 32 | Search box and keyword fields bring up the keyboard | Works (CI) | Screenshot 04-text-field. |
| — | Apple Pencil, real-finger pinch, two-finger tap, long-press | Not checked | Need the real iPad (see Needs Ivan). |

**Still broken or hard to use, most painful first** (queued below):
1. Small touch targets: curve points, mask handles, crop corners, the radial mask's rotate knob,
   rating stars.
2. Handles are grabbed where the finger is after it moved a little, not where it landed (crop,
   targeted adjustment, first bit of a brush or spot stroke).
3. Sync to several photos: no way to pick several photos by touch.
4. Removing a tone-curve point needs a precise double-tap.
5. Crop has no Done button (double-tap inside the crop, or Return).
6. Hover-only things: preset preview, snapshot preview, the colour wheels' eye, the reset hints.
7. ~~The on-screen keyboard covers the bottom of the screen~~ (keyboard avoidance, 2026-10-20
   morning, DECISIONS.md 45).

**In the audit fixes:** keyboard only for a tapped text box; ⌄ Before/After menu; two-finger undo,
three-finger redo; up/down drags on sliders scroll the panel; mask pins always visible; AltStore
Classic setup steps and source fix.

## Latest session summary

**2026-10-26 (night 4, 02:54–03:40 Berlin, unattended, last run of the night)**
1. Main run 70 green: **released 1.70** (the loupe shows "Select Several: N photos | Done" above the filmstrip).
2. Fixed run 69's keyboard flicker: a text box low on the screen (Info panel › Location) kept making the keyboard drop and come back. The keyboard now stays up while the box is being edited, and the box scrolls back into view (DECISIONS 67).
3. Branch run 71 green (build, unit tests incl. a new one, clippy, whole simulator tour); **merged `p4-keyboard-check`** into main; main's run releases about 1.72.
4. Not checked yet: run 71's screenshot of the low box with the keyboard (the report wasn't published in time). Next session looks at it first.
5. Next: check main's run and that screenshot, make the low-field tour steps hard, then rotation / Split View in the tour.

**2026-10-26 (night 3, 01:55–02:40 Berlin, unattended)**
1. Main run 68 green: **released 1.68** (long-press a version or History step → preview + Restore).
2. Run 67 green and its shot shows the loupe's "Select Several: 3 photos | Done" pill: **merged `p4-film-select`**; main's run releases about 1.70.
3. New branch `p4-keyboard-check` (DECISIONS 66): the tour taps a field low enough for the keyboard to cover it and expects the new log line "lifting the UI above the on-screen keyboard" (soft until seen); the Sync toast lasts 3 s so the tour shot can catch it.
4. Branch run 69 failed and **found a real bug**: a text field low on the screen makes the keyboard flicker (the UI lifts, the keyboard drops, again every 150 ms). Not merged; cause and fix ideas in HANDOFF.md.
5. Next: check main's run 70, fix the flicker on `p4-keyboard-check`, then merge.

**2026-10-26 (night 2, 00:55–01:40 Berlin, unattended)**
1. Main run 64 green: **released 1.64** (long-press a preset → preview + Apply).
2. Tour fix on `p4-snapshot-preview`: `panel.edit` toggled the Edit panel shut, so the slider long-press found no slider; the tour now opens it with `ui.set` and the long-press steps are hard checks (DECISIONS 65). Branch run 65's tour passed.
3. New branch `p4-film-select`: while Select Several is on, the loupe shows a pill above the filmstrip ("Select Several: 2 photos" + Done), with a unit test and tour shot `10-select-several-loupe` (DECISIONS 64). Run 66 failed to compile (new translation message); fixed, run 67 going.
4. Run 65 green and its screenshots right: **merged `p4-snapshot-preview`** (version / History preview by long-press); main's run releases about 1.68. `p4-film-select` (run 67) still waits.
5. Next: check main's run, merge `p4-film-select` after its screenshot, then the keyboard-shrink CI check.

**2026-10-25/26 (night 1, 23:55–00:40 Berlin, unattended)**
1. Main run 60's re-run was green: **released 1.60** (long-press a slider's name → Reset). The first attempt's log-check failure was a flake; the tour's `checkLog` now waits up to 5 s.
2. **Merged `p4-preset-preview`** into main (long-press a preset → preview + Apply); main's run 64 releases about 1.64.
3. New branch `p4-snapshot-preview`: long-press a **version** or a **History step** → the photo previews it while the menu is open, *Restore* at the top (DECISIONS 63). Both tests passed and branch run 63 is green (preset and History previews seen in the screenshots).
4. Tour shots of all the new long-press menus (`11-*.jpg`), as soft steps until their screenshots are checked.
5. Next: merge `p4-snapshot-preview` after main's run 64; fix the tour's slider-menu step (its label wasn't on screen).

**2026-10-25 (evening 5, 22:55–23:40 Berlin, unattended)**
1. Main run 57 green, **released 1.57** (Select Several strip with Sync Settings and Done).
2. Branch run 56 green; **merged `p4-reset-menu`** (long-press a slider's name → Reset); main's run 60 failed on a search-keyboard log check that passed on the same code before (flake?), re-run once.
3. Sync Settings checked by a new app-level test (photo 2's exposure follows photo 1, toast says so); the missing toast in run 54's screenshot was timing (1.4 s toast), tour shot now 0.3 s after the tap.
4. New: long-press a preset → the photo previews it while the menu is open, *Apply* at the top (DECISIONS 62); its test passed in run 59. Branch `p4-preset-preview`, run 61 going.
5. Next: merge `p4-preset-preview` when run 61 is green, then snapshot preview by long-press.

**2026-10-25 (evening 4, 21:55–22:40 Berlin, unattended)**
1. Main run 53 green, **released 1.53** (radial knob, star touch area, Select Several, colour-wheel eye).
2. Branch `p4-select-bar` run 52 green (strip + 3 selected, then strip gone; screenshots checked).
3. New *Sync Settings* button in the Select Several strip (DECISIONS 60), branch run 54 green (button seen in the screenshot); **merged `p4-select-bar` + `p4-select-sync` into main**, whose run releases the next version.
4. Branch `p4-reset-menu`: long-press a slider's name → *Reset <name>* (DECISIONS 61). Run 55's raw-frame unit test failed; rewritten as an app-level test in `tests_curve`, run 56 going.
5. Next: check main's run and run 56, merge `p4-reset-menu`, then preset preview by long-press.

**2026-10-25 (evening 3, 20:54–21:40 Berlin, unattended)**
1. Run 49's tour failure was the test, not the app: `thumb:2` sat above the screen. New tour step `clickVisible` taps photos that are fully on screen; branch run 50 green, 3 photos selected (screenshot checked).
2. Run 48's last-step failure was a flake (re-run green); the Add-to-Photos step after the share sheet no longer fails the tour.
3. **Merged `p4-select-several` into main** (radial knob, bigger star touch area, Select Several, colour-wheel eye): main's run releases about 1.5x.
4. Branch `p4-select-bar`: a strip "Tap photos to add or remove them · Done" while Select Several is on (DECISIONS 59). Run 51 showed the strip fine but the tour tapped under it; fixed, run 52 going.
5. Next: merge `p4-select-bar` when run 52 is green, then hover-only leftovers.

**2026-10-25 (evening 2, 19:54–20:35 Berlin, unattended)**
1. Main run 47 green, **released 1.47** (touch grabs, curve Delete Point, Crop Done; screenshots checked). Upstream still 0.9.1.
2. Branch `p4-touch-2`: radial rotate knob bigger and 44 pt out, targeted adjustment at the press, rating stars 36 pt tall to touch. Built fine; tour failed only at the last (Photos) step, re-run started.
3. Branch `p4-select-several` (includes the above): *Select Several* in the photo long-press menu, so taps pick several photos for Sync, with a CI tour check; colour-wheel eye always shown on iPad.
4. Nothing new merged into main this session: see HANDOFF.md for the run results to check and the merge step.
5. Next: merge those branches, then hover-only leftovers (preset preview, reset hints).

**2026-10-25 (evening, 18:55–19:40 Berlin, unattended)**
1. Main was green (run 44, **released 1.44**: the editing audit). No new upstream release (still 0.9.1).
2. Touch: curve points, crop handles and mask pins/handles are grabbed within 22 pt on iPad; a crop drag starts where the finger landed (DECISIONS.md 53).
3. Long-press a tone-curve point → *Delete Point* (DECISIONS.md 54); a *Done* button in the Crop panel (56).
4. CI now also runs the upstream curve unit tests (55). Branch runs 45 (curve only) and 46 (everything) — see HANDOFF.md for their result and whether it got merged.
5. Next: rating stars and the radial rotate knob (still small), then picking several photos for Sync.

**2026-10-20 (interactive, about 09:40 Berlin)**
1. Uploaded the 2026-10-19 audit session's 8 commits (they couldn't be pushed that night) to branch
   `audit-2026-10-19`, unchanged (file hashes checked against the patch series).
2. Merged main (this morning's keyboard avoidance) into it: one tour conflict (kept both sides);
   the audit's decisions renumbered 47–52.
3. Branch run 41: everything worked (no keyboard while editing, every Before/After view) except
   that tapping the search box no longer brought the keyboard up. Two fixes: a tap counts for 4
   frames as well as 0.8 s (the simulator draws slowly), and the tapped box is the focused
   widget's rectangle (egui's IME rect is only the text cursor, run 42's log). Run 43 all green
   (`04-text-field` shows the keyboard, `07-*` show none, `08-*` show every view). Merged into main.

**2026-10-19 (editing audit, evening, unattended)**
1. Couldn't upload (no write access); saved as a patch series in the Project (DECISIONS.md 52).
2. Recorded Ivan's two iPad findings (FEEDBACK.md), his AltStore Classic setup (SETUP.md), and
   fixed the source for AltStore 2.x (it now lists the Photos permission).
3. Wrote fixes: keyboard only for a tapped text box; ⌄ menu with every Before/After view;
   two-finger undo / three-finger redo; vertical drags on sliders scroll; mask pins visible.
4. Full editing audit: table at the top of this file.

**2026-10-20 (morning, 08:40–09:20 Berlin, unattended)**
1. Main run 38 green, **released 1.38** (on-screen keyboard); the hardware-key checks passed again and are now hard checks.
2. **Keyboard avoidance:** when the on-screen keyboard would cover the text field you're typing in, the whole UI now moves up above it (DECISIONS.md 45). Fields higher up (search box, caption) leave the layout alone.
3. CI now types into the search box and checks the text arrives once (it did), and focuses the caption with the keyboard up (screenshot checked).
4. Branch run 39 all green; merged into main, whose run should release about 1.40.
5. Next: make CI focus a field low enough to trigger the shrink, and double-check the measured keyboard height.

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
- [~] **4. Input:** done: long-press → context menu, safe area from UIKit, top gap for the multitasking button, pinch-zoom/pan in the loupe (tested in CI with egui events), Pencil skips the long-press wait (brush strokes; DECISIONS.md 39), brush stroke in the CI tour, hardware keyboard read from UIKit with ⌘ as command (DECISIONS.md 40, 41; keys reach the app in CI run 33), on-screen keyboard (DECISIONS.md 43) and keyboard avoidance (DECISIONS.md 45). To do: real-finger and Pencil check on the iPad; rotation, Split View / Stage Manager resizing
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

Main run 70 green, released **1.70** (2026-10-26: Select Several pill in the loupe). Main's run for
the `p4-keyboard-check` merge (low text box: no keyboard flicker, DECISIONS 67) releases about
**1.72**. CI checks build, fmt, unit tests, clippy, the simulator tour (sliders, radial and brush
masks, pinch/pan, hardware keys with rating 5 and ⌘Z undo as hard checks, search box with
on-screen keyboard and typing, caption and a low Info field with the keyboard, Select Several and
Sync, the long-press menus, background export, Share button and sheet, Add to Apple Photos as a
soft check), the export file and the log file.

## Next 3 tasks

1. **Check run 71's / main's `04-low-field-keyboard` shot** (keyboard up, Location box visible
   above it, "lifting the UI" logged once), then make those tour steps hard (HANDOFF.md).
2. **Rotation and Split View / Stage Manager resizing in the tour** (`simctl` can't rotate: try
   the Simulator menu via osascript on the CI Mac).
3. **A 24 MP raw memory test in the simulator** (CC0 ARW/RW2 from `cargo xtask corpus`).

Also queued:
- Check the measured keyboard height (257 pt logged vs. the screenshot).
- Delete merged branches (no tool for it from the sandbox).

## Needs Ivan

### This week: what to test on the iPad (start here)

First update RawPad: plug the iPad into the PC, make sure AltServer is running, open AltStore ›
**My Apps** › **Update** next to RawPad (pick the newest: 1.70, or 1.72 once it shows up). Then try
these and tell Claude in chat what you see (a sentence per item is plenty; "works" is fine):

1. **Keyboard stays away when editing** (your note from 2026-10-19): open a photo, move several
   sliders, tap the photo. The keyboard must **not** appear. Then tap **Search Photos** at the
   top: now it should appear. Tap the photo: it should go away.
2. **Before/After split** (your other note): in Develop, tap the small **⌄** right of the
   Before/After button (bottom right). Try **Split**, **Top/Bottom**, **Split Top/Bottom**,
   **Show Original**. Does each one show before and after? If not, say what you see.
3. **A box low on the screen (new in 1.72):** open a photo, tap the **ⓘ** (Info) button, tap
   **Location**. The keyboard should come up **once and stay**, with the Location box visible
   above it (in 1.70 it may flicker). Type a word, then tap the photo.
4. **Pick several photos:** in the grid, press and hold a photo › **Select Several**, tap two
   more photos, then **Sync Settings** in the strip under the title: do the other photos get the
   first photo's edits? Open a photo while it's on: the strip above the filmstrip should say
   "Select Several: 3 photos" with **Done**.
5. **Press and hold** (half a second): a slider's **name** → *Reset*; a **preset** → the photo
   previews it, *Apply*; a row in **Versions** or **History** → the photo previews it, *Restore*.
6. **Undo with fingers:** after a slider change, tap the photo with **two fingers** at once:
   undone. **Three fingers**: redone.
7. **Pencil and fingers:** paint a brush mask with the Pencil; pinch to zoom; scroll the Edit
   panel with a finger starting on a slider (the slider should not move).

If anything crashes or freezes: Files › On My iPad › RawPad › Logs › press and hold
**lightcraft.log** › Share, and send it to Claude in chat.

### Older notes

**Re-test after the next update (1.47 is out now; AltStore should list it), in this order,
and tell Claude what you see:**
1. **Keyboard:** open a photo, move several sliders, paint a brush mask with the Pencil, tap the
   photo. The keyboard must **not** appear. Then tap **Search Photos**: it must appear. Tap a
   slider's number (right of its name): it should appear so you can type a value.
2. **Before/After:** in Develop, tap the small **⌄** right of the Before/After button (bottom
   right). Try each entry: Split, Top/Bottom, Split Top/Bottom, Show Original. Does each one show
   the photo before and after your edits? If split still "doesn't work", say what you see (one
   photo? no line? the same picture on both sides?).
3. **Undo:** after a slider change, tap the photo with **two fingers** at once (quick tap): the
   change should be undone. Three fingers: redo.
4. **Scrolling:** scroll the right-hand Edit panel with a finger, starting on a slider. The slider
   should not move.
5. Things only the real iPad can show: Pencil strokes, pinch-zoom with fingers, press-and-hold
   menus.
7. **New in 1.53 (AltStore lists it now):** in the photo grid,
   press and hold a photo → **Select Several** (top of the menu). Then each tap adds a photo or
   takes it out (the title row says "3 selected"). Long-press again → **Done Selecting**. With
   several photos picked, the right panel shows Quick Develop for all of them. Also: the round
   mask's rotate knob is bigger and further out; rating stars are easier to tap.
8. **New in the version after 1.53 (about 1.57, once AltStore lists it):** while Select Several is
   on, a strip under the grid title says "Tap photos to add or remove them" with **Sync Settings**
   (copies the edits of the photo you long-pressed first to the other picked photos) and **Done**.
9. **New in the version after 1.57 (about 1.60):** in Develop, press and hold a slider's
   **name** (e.g. "Exposure") → **Reset Exposure** puts that one slider back to zero.
10. **New in 1.64 (AltStore should list it now):** press and hold a
   preset in the Presets list: the photo shows how the preset looks while the menu is open;
   **Apply** at the top of the menu applies it. Tapping anywhere else closes the menu and leaves
   the photo as it was.
11. **New in 1.68 (AltStore should list it now):** in the **Versions** panel (right-hand strip) and the
   **History** panel, press and hold a row: the photo shows that version or step while the menu
   is open; **Restore** at the top goes back to it. Also: with Select Several on, the Develop
   view shows "Select Several: N photos" with a **Done** button just above the filmstrip
   (that part arrives in the version after 1.68, about 1.70).
6. **New in 1.47:** in the Tone Curve,
   drag a point with your finger (it should be easy to catch); press and hold a point for half a
   second → **Delete Point**. In Crop, there is now a **Done** button next to Reset. Crop corners
   and mask pins should be easier to grab.

**Install:** you use **AltStore Classic** (your iPad blocks VPN apps, so SideStore can't work).
Steps are in `SETUP.md` › *Install with AltStore Classic*. Updates and the weekly refresh: plug the
iPad into the PC with the cable, make sure AltServer runs, then AltStore › My Apps.

**Earlier list (still useful):** install SideStore (steps 1–2 below) if it isn't there yet, update RawPad
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
   - **New (1.40 or later), keyboard avoidance:** open a photo, tap the **ⓘ** (Info) button at
     the bottom right, scroll the panel down and tap a box near the bottom (e.g. Creator): when
     the keyboard comes up, does the whole screen move up so you can still see the box you're
     typing in?
   - **New (1.38 or later), on-screen keyboard:** without a keyboard attached, tap the
     **Search Photos** box at the top: the iPad keyboard should come up. Type a word: does it
     appear in the box, and does the delete key work? Tap somewhere on the photo: does the
     keyboard go away? (Also try a keyword field in the right panel's info section.)
   - **New (1.35 or later), keyboard:** if you have a keyboard for the iPad, open a photo and
     press **5** (five stars?), then **⌘Z** (undo: back to the old stars?). Also try ⌘Z after a
     slider change. Tell Claude what happens; the log (see above) shows `keyboard (UIKit):` lines.
   - Anything slow, frozen or crashing: say which photo type (ARW or RW2) and what you did.
