#!/bin/bash
set -euo pipefail

# Keep the desktop's content-addressed engine staging and bootstrap identity
# consistent by changing a copied app, rather than overwriting its cached engine.
PACKAGE_DIR="$(cd -- "$(dirname -- "$0")" && pwd)"
SOURCE_ENGINE="${1:-$PACKAGE_DIR/msc}"
EXPECTED_VERSION="${2:-$(cat "$PACKAGE_DIR/MSC-VERSION")}"
OUTPUT_DIR="${3:-$PACKAGE_DIR/patched}"
SOURCE_APP="${4:-/Applications/Minecraft Server Controller.app}"
PATCHED_APP="$OUTPUT_DIR/Minecraft Server Controller.app"

fail() { printf '%s\n' "$*" >&2; exit 1; }
[[ "$(uname -s):$(uname -m)" == Darwin:x86_64 ]] || fail 'Run this package on the Intel Mac.'
[[ -f "$SOURCE_ENGINE" && ! -L "$SOURCE_ENGINE" ]] || fail 'The replacement engine is missing or is a symbolic link.'
[[ -d "$SOURCE_APP" && ! -L "$SOURCE_APP" ]] || fail 'The installed MSC app is missing or is a symbolic link.'
[[ ! -e "$PATCHED_APP" ]] || fail "Output already exists: $PATCHED_APP. Choose a new output folder."
/usr/bin/lipo "$SOURCE_ENGINE" -verify_arch x86_64
APP_VERSION="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$SOURCE_APP/Contents/Info.plist")"
[[ "$APP_VERSION" == "$EXPECTED_VERSION" ]] || fail "This patch requires MSC $EXPECTED_VERSION; the installed app is $APP_VERSION."
[[ -f "$SOURCE_APP/Contents/Resources/agent/msc" ]] || fail 'The installed app has no bundled engine.'

mkdir -p "$OUTPUT_DIR"
/usr/bin/ditto "$SOURCE_APP" "$PATCHED_APP"
/usr/bin/install -m 0755 "$SOURCE_ENGINE" "$PATCHED_APP/Contents/Resources/agent/msc"
/usr/bin/codesign --force --sign - "$PATCHED_APP/Contents/Resources/agent/msc"
# Only the replaced engine and outer app need new signatures. Leave the Swift
# helper's existing signature and virtualization entitlement intact.
/usr/bin/codesign --force --sign - --preserve-metadata=entitlements,flags,runtime "$PATCHED_APP"
/usr/bin/codesign --verify --deep --strict "$PATCHED_APP"
printf '\nPrepared: %s\n' "$PATCHED_APP"
printf '%s\n' 'Stop Minecraft in MSC and wait for Stopped, then quit MSC.'
printf '%s\n' 'Copy the prepared app into Applications, replacing MSC. Open it and use Repair engine before starting Minecraft.'
printf '%s\n' 'The installed app, running services, settings and worlds have not been changed by this preparation.'
/usr/bin/open -R "$PATCHED_APP"
