#!/usr/bin/env python3
"""Add a release to the SideStore/AltStore source (sidestore/source.json).

Usage: update_source.py <version> <build> <tag> <ipa_path> <notes>
The newest version goes first; the last 10 are kept. SideStore checks `version` and
`buildVersion` against the .ipa's CFBundleShortVersionString / CFBundleVersion.
"""
import datetime
import json
import os
import sys

REPO = "Ivanmagpelmeni/rawpad"
RAW = f"https://raw.githubusercontent.com/{REPO}/main"
PATH = os.path.join(os.path.dirname(__file__), "..", "sidestore", "source.json")


def base():
    return {
        "name": "RawPad",
        "identifier": "io.github.ivanmagpelmeni.rawpad.source",
        "subtitle": "LightCraft photo editor for iPad",
        "description": "Personal builds of RawPad, a native iPadOS port of LightCraft (open-source raw developer and photo library).",
        "iconURL": f"{RAW}/ios/icon/AppIcon-1024.png",
        "website": f"https://github.com/{REPO}",
        "tintColor": "#FFB85C",
        "apps": [
            {
                "name": "RawPad",
                "bundleIdentifier": "io.github.ivanmagpelmeni.rawpad",
                "developerName": "Ivanmagpelmeni",
                "subtitle": "Raw developer and photo library",
                "localizedDescription": "LightCraft on iPad: library, culling, develop sliders, masks, presets and export. "
                "Based on LightCraft by the ArtCraft team; not an ArtCraft product.",
                "iconURL": f"{RAW}/ios/icon/AppIcon-1024.png",
                "tintColor": "#FFB85C",
                "category": "photo-video",
                "screenshots": [],
                "versions": [],
                "appPermissions": {"entitlements": [], "privacy": {}},
            }
        ],
        "news": [],
    }


def main():
    version, build, tag, ipa, notes = sys.argv[1:6]
    try:
        with open(PATH) as f:
            source = json.load(f)
    except (OSError, ValueError):
        source = base()
    app = source["apps"][0]
    entry = {
        "version": version,
        "buildVersion": build,
        "date": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "localizedDescription": notes,
        "downloadURL": f"https://github.com/{REPO}/releases/download/{tag}/RawPad.ipa",
        "size": os.path.getsize(ipa),
        "minOSVersion": "17.0",
    }
    app["versions"] = [entry] + [v for v in app.get("versions", []) if v.get("version") != version][:9]
    # older AltStore clients read these top-level fields
    app.update({k: entry[k] for k in ("version", "downloadURL", "size")})
    app["versionDate"] = entry["date"]
    app["versionDescription"] = notes
    os.makedirs(os.path.dirname(PATH), exist_ok=True)
    with open(PATH, "w") as f:
        json.dump(source, f, indent=2)
        f.write("\n")
    print(f"source.json → {version} ({tag})")


if __name__ == "__main__":
    main()
