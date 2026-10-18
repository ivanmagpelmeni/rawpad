#!/usr/bin/env bash
# Build RawPad.app for the iPad simulator or a device (macOS + Xcode command line tools).
#
#   ios/build.sh sim      → target/ipad/sim/RawPad.app       (ad-hoc signed, for `simctl install`)
#   ios/build.sh device   → target/ipad/device/RawPad.ipa    (unsigned; SideStore signs it on the iPad)
#
# RAWPAD_BUILD (default: GITHUB_RUN_NUMBER, else 1) becomes the app version 1.<build>.
set -euo pipefail
cd "$(dirname "$0")/.."

kind="${1:?usage: ios/build.sh sim|device}"
case "$kind" in
  sim)    target=aarch64-apple-ios-sim; platform=iPhoneSimulator ;;
  device) target=aarch64-apple-ios;     platform=iPhoneOS ;;
  *) echo "unknown kind: $kind" >&2; exit 2 ;;
esac

build="${RAWPAD_BUILD:-${GITHUB_RUN_NUMBER:-1}}"
version="1.${build}"
export IPHONEOS_DEPLOYMENT_TARGET=17.0

cargo build --release --target "$target" -p rawpad-ios --bin rawpad

out="target/ipad/$kind"
app="$out/Payload/RawPad.app"
rm -rf "$out"
mkdir -p "$app"
cp "target/$target/release/rawpad" "$app/rawpad"
strip -x "$app/rawpad" 2>/dev/null || true
cp ios/icon/AppIcon60x60@2x.png ios/icon/AppIcon76x76@2x~ipad.png ios/icon/AppIcon83.5x83.5@2x~ipad.png "$app/"
sed -e "s/__VERSION__/$version/" -e "s/__BUILD__/$build/" -e "s/__PLATFORM__/$platform/" ios/Info.plist > "$app/Info.plist"
plutil -lint "$app/Info.plist"
printf 'APPL????' > "$app/PkgInfo"

if [ "$kind" = sim ]; then
  # the simulator only launches signed apps; an ad-hoc signature is enough
  codesign --force --sign - --timestamp=none "$app"
  rm -rf "$out/RawPad.app" && mv "$app" "$out/RawPad.app" && rmdir "$out/Payload"
  echo "built $out/RawPad.app ($version)"
else
  (cd "$out" && zip -qry RawPad.ipa Payload)
  echo "built $out/RawPad.ipa ($version, $(du -h "$out/RawPad.ipa" | cut -f1))"
fi
echo "$version" > "$out/version.txt"
