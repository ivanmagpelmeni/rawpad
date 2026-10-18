# Setting up RawPad on your iPad

You do this once, on a Windows PC. After that, everything happens on the iPad. This follows the
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
