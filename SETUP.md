# Setting up RawPad on your iPad

You do this once, on a Windows PC. **Managed iPad (VPN apps blocked)?** Skip to *Install with AltStore Classic* below. After that, everything happens on the iPad. This follows the
official SideStore guide (docs.sidestore.io, checked 2026-10-18). If a screen looks different,
the official guide wins. Tell Claude what you saw.

**Never share your Apple ID password with anyone, including Claude.** You type it only into
iloader and SideStore.

---

## Part A: prepare (about 10 minutes)

**On the iPad:**
1. Make sure the iPad has a passcode: **Settings › Face ID/Touch ID & Passcode**.
2. Open the **App Store**. Search for **LocalDevVPN**, then install it (free).

**On the Windows PC:**

3. Install **iTunes from Apple's website**, not from the Microsoft Store:
   1. Go to **apple.com/itunes**.
   2. Scroll to *Looking for other versions?* and click **Windows**.
   3. Download and install it.

   (If that doesn't work, use the Microsoft Store version or Apple's **Apple Devices** app.)
4. Go to **iloader.app**. Download the **Windows** version and install it.

## Part B: install SideStore with iloader (Windows PC)

1. Connect the iPad to the PC with a USB cable.
2. If the iPad asks **Trust This Computer?**, tap **Trust** and enter your passcode.
3. Open **iloader**.
4. Sign in with your Apple ID. It's case-sensitive.
5. Select your iPad.
6. Click **Install SideStore (Stable)** and wait until iloader says it's done.
7. Unplug the iPad.

## Part C: allow SideStore (on the iPad)

1. Open **Settings › General › VPN & Device Management**.
2. Under **Developer App**, tap the entry with your Apple ID.
3. Tap **Trust "…"**, then **Allow & Restart**, and enter your passcode.
4. After the restart, open **Settings › Privacy & Security**.
5. Scroll to the bottom, turn on **Developer Mode**, and let the iPad restart.
6. Confirm **Turn On** when the iPad asks after the restart.

## Part D: first SideStore start (on the iPad)

1. Open **LocalDevVPN** and tap **Connect**. Allow the VPN configuration if asked.
2. Open **SideStore** and sign in with the **same** Apple ID.
3. Go to **My Apps**. Tap the **7 DAYS** button next to SideStore to refresh it once.
4. If it asks to revoke or create a certificate, tap **Yes** or **Refresh Now**.
5. The iPad goes back to the Home Screen with a notification. SideStore works again a few seconds
   later.

## Part E: add RawPad

The first RawPad release appears once phase 3 is ready. `PROGRESS.md` → *Needs Ivan* says when.

1. Make sure **LocalDevVPN** is connected.
2. In SideStore, open the **Sources** tab and tap **+**.
3. Paste this address and confirm:
   ```
   https://raw.githubusercontent.com/Ivanmagpelmeni/rawpad/main/sidestore/source.json
   ```
4. Open the **RawPad** source, then tap **RawPad › Free** (or **Get**) to install it.
5. RawPad appears on the Home Screen.

**Plan B,** if the source doesn't work:
1. In Safari, open **github.com/Ivanmagpelmeni/rawpad/releases**.
2. Under the newest release, tap **RawPad.ipa** to download it.
3. In SideStore, go to **My Apps**, tap **+**, and pick the file from **Downloads**.

## Install with AltStore Classic (managed iPads)

**This is what Ivan uses (2026-10-19).** Some iPads are looked after by a company or school
("managed", with MDM). Those can block VPN apps, and then SideStore can't work, because it needs
the LocalDevVPN app. AltStore Classic needs no VPN: instead, a small program on your Windows PC
(AltServer) signs the apps when the iPad is plugged in with a cable. Parts A–D above are not
needed in that case.

**Once, on the Windows PC:**
1. Install **iTunes** and **iCloud** from Apple's website (not the Microsoft Store versions).
2. Download **AltServer for Windows** from **altstore.io** and install it. It runs as a small icon
   near the clock.
3. Connect the iPad with the USB-C cable. If it asks **Trust This Computer?**, tap **Trust**.
4. Click the AltServer icon › **Install AltStore** › pick your iPad, and sign in with your Apple ID
   when asked (only there, never in a chat).
5. On the iPad, if needed: **Settings › General › VPN & Device Management** › trust your Apple ID,
   and **Settings › Privacy & Security › Developer Mode** › on.

**Add RawPad (on the iPad, with the cable in and AltServer running on the PC):**
1. Open **AltStore** › **Sources** › **+**.
2. Paste this address and confirm:
   ```
   https://raw.githubusercontent.com/Ivanmagpelmeni/rawpad/main/sidestore/source.json
   ```
   (The same address works for SideStore and AltStore.)
3. Open the **RawPad** source and tap **Free** (or **Get**).

**Every week, and for every update:**
- Plug the iPad into the PC with the cable and make sure AltServer is running (the icon near the
  clock).
- Open **AltStore › My Apps**. Tap **Refresh All** (keeps RawPad working for another 7 days), or
  **Update** next to RawPad when a new version is listed.
- Wi-Fi refresh only works when the PC and the iPad are on the same network. Yours aren't, so
  always use the cable.
- If AltStore says the source or the app's "permissions don't match", tell Claude: the source
  lists what RawPad asks for (adding photos to Photos), and AltStore checks that list.

## Using RawPad: getting photos in and out

- **Import:**
  1. Tap **+** in the top bar (Import Photos…).
  2. Pick photos in the Files window: iPad, iCloud Drive, or the SD card under *Locations*.
  3. Confirm **Import**.

  For a whole folder or card, use **Menu › File › Import from Folder…**.
- **Imported photos are copied into RawPad's library.** You can remove the SD card afterwards.
- **Export:**
  1. Use **Export**.
  2. The JPEGs land in the **Files** app under **On My iPad › RawPad › Exports**.
  3. From there, tap a file, then **Share › Save Image** to put it in the Photos app.

  (Direct saving to Photos is coming.)

## Every week: keep it working

- Free Apple IDs sign apps for **7 days**. SideStore refreshes them, but the **VPN must be
  connected** for that.
- Easiest habit, once a week:
  1. Open **LocalDevVPN** and tap **Connect**.
  2. Open **SideStore** and go to **My Apps**.
  3. Tap **Refresh All**.
- **Updates:** SideStore shows an update for RawPad when a new release is out. Tap **Update**,
  with the VPN on. Your library and edits are kept: same app, same data.

## If something goes wrong

- **RawPad won't open, or says it's no longer available:** its signature expired. Connect
  LocalDevVPN, open SideStore, and refresh.
- **SideStore itself expired or won't open:** redo Part B on the Windows PC. Your apps and data
  stay.
- **SideStore says the device UDID can't be determined, or the pairing file is invalid:** this
  can happen after an iPadOS update. Connect the iPad to the PC, open iloader, and use its
  pairing-file option to replace the pairing file. Ask Claude if unsure.
- **"Maximum app limit reached":** a free Apple ID allows 3 sideloaded apps, including SideStore.
- **Don't delete RawPad to "fix" it.** Deleting the app deletes its photo library. Export
  anything you need first, or refresh or update instead.
