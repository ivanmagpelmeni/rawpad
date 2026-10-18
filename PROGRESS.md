# Progress

## Latest session summary

**2026-10-18 (kickoff, interactive)**
1. Kickoff interview done, approach A approved, and the brief written (Project file
   `lightcraft-ipad-brief.md`).
2. Fork scaffolded:
   - ArtCraft marks removed.
   - Repo-memory docs added.
   - Upstream desktop workflows removed.
   - Own app icon.
3. iOS host crate `apps/rawpad-ios`: winit (UIKit) + egui-winit + egui-wgpu (Metal) hosting the
   LightCraft UI, library in the app container, saves on background, control channel for CI.
4. CI `.github/workflows/ipad.yml`:
   - Builds sim and device, plus fmt and clippy.
   - Runs an iPad simulator tour with screenshots.
   - Publishes logs and screenshots to the `ci-artifacts` branch.
   - Releases the `.ipa` and updates the SideStore source.
5. Next: get the first CI run green (sandbox can't compile, so CI is the compiler).

## Phases

- [x] **0. Kickoff:** brief, public fork, docs, CI skeleton, SideStore steps (`SETUP.md`)
- [ ] **1. Engine compiles for iOS** (sim + device) in CI; headless render in the simulator matches
      the CPU reference
- [ ] **2. App shell** launches on the iPad simulator, shows the LightCraft UI on Metal with the
      demo library; CI screenshots
- [ ] **3. First install on Ivan's iPad via SideStore** ← highest priority after 2
- [ ] **4. Input:** touch → pointer mapping, pinch-zoom/pan in loupe, Pencil (USB-C, no pressure)
      for brush masks, hardware-keyboard shortcuts; rotation, Split View / Stage Manager resizing,
      safe areas
- [ ] **5. Files:** import from Files / SD card (UIDocumentPicker through `Services::picker`);
      export JPEG to Photos + Files, original raw to Files, share sheet
- [ ] **6. Performance and memory on the A16 (~6 GB):**
  - tiling, preview downscaling, a smaller thumbnail cache
  - memory-warning handling and `malloc_zone_pressure_relief` on iOS
  - backgrounding and crash safety; 24 MP ARW/RW2 tests
- [ ] **7. Parity sweep against upstream** (minus AI and the second display):
  - touch polish
  - hide ArtCraft Discord/website menu links in the iPad build
  - craft-fonts in release builds
  - routine upstream merges

## Current status

Phase 1 and 2 code is written but **not compiled yet**: the first CI run is the first compile.

## Next 3 tasks

1. Push and read the first CI run. Fix compile errors in `apps/rawpad-ios` until
   `ios/build.sh sim` and `device` succeed.
2. Get the simulator tour green and look at the screenshots: UI visible? safe areas? scale?
3. When green on main, check that a release and `sidestore/source.json` were published, then move
   "Install RawPad" to *Needs Ivan*.

## Needs Ivan

1. **Install SideStore** once from a Windows PC: `SETUP.md` parts A–D. Do this any time before
   phase 3.
2. *(Later, phase 3)* Add the RawPad source in SideStore: `SETUP.md` part E.
