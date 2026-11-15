#!/usr/bin/env bash
set -Eeuo pipefail

WORKSPACE_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
OUTPUT_ROOT="${1:-$WORKSPACE_ROOT/target/release-artifacts}"
RUST_TARGET="${2:-x86_64-apple-darwin}"
PLATFORM_LABEL="${3:-macos-x86_64}"
ENGINE_PACKAGE="$WORKSPACE_ROOT/crates/msc-engine/Cargo.toml"
STAGED_ENGINE="$WORKSPACE_ROOT/clients/desktop-web/src-tauri/target/package/agent"

fail() {
  printf 'MSC macOS headless build: %s\n' "$1" >&2
  exit 1
}

[[ "$(uname -s)" == "Darwin" ]] || fail "run the macOS headless build on macOS"
case "$RUST_TARGET:$PLATFORM_LABEL:$(uname -m)" in
  x86_64-apple-darwin:macos-x86_64:x86_64)
    INCLUDE_SIDECAR=true
    ;;
  aarch64-apple-darwin:macos-aarch64:arm64)
    INCLUDE_SIDECAR=false
    ;;
  *)
    fail "target $RUST_TARGET and label $PLATFORM_LABEL do not match this macOS runner"
    ;;
esac
command -v cargo >/dev/null 2>&1 || fail "cargo is required"
command -v tar >/dev/null 2>&1 || fail "tar is required"

VERSION="$(awk -F'"' '/^[[:space:]]*version[[:space:]]*=/ { print $2; exit }' "$ENGINE_PACKAGE")"
[[ -n "$VERSION" ]] || fail "could not read the msc-engine version"

cd "$WORKSPACE_ROOT"
cargo build --release --locked --no-default-features --target "$RUST_TARGET" -p msc-engine
cargo build --release --locked --target "$RUST_TARGET" \
  --manifest-path "$WORKSPACE_ROOT/tools/world-map-proof/Cargo.toml"

SOURCE_BINARY="$WORKSPACE_ROOT/target/$RUST_TARGET/release/msc"
[[ -x "$SOURCE_BINARY" ]] || fail "release binary is missing: $SOURCE_BINARY"
if [[ "$INCLUDE_SIDECAR" == true ]]; then
  [[ -x "$STAGED_ENGINE/sidecar/BedrockSidecar" ]] || fail "staged BedrockSidecar is missing"
  [[ -f "$STAGED_ENGINE/sidecar/vmlinuz-kata" ]] || fail "staged Bedrock kernel is missing"
  [[ -f "$STAGED_ENGINE/sidecar/appliance-initramfs.gz" ]] || fail "staged Bedrock initramfs is missing"
else
  [[ ! -e "$STAGED_ENGINE/sidecar" ]] || fail "Apple Silicon staging unexpectedly contains Bedrock sidecar files"
fi

PLATFORM_DIR="$OUTPUT_ROOT/$PLATFORM_LABEL"
PACKAGE_ROOT="$OUTPUT_ROOT/.macos-package"
ARCHIVE="$OUTPUT_ROOT/msc-headless-${VERSION}-${PLATFORM_LABEL}.tar.gz"
rm -rf "$PACKAGE_ROOT"
mkdir -p "$PLATFORM_DIR" "$PACKAGE_ROOT"
install -m 0755 "$SOURCE_BINARY" "$PLATFORM_DIR/msc"
install -m 0755 "$SOURCE_BINARY" "$PACKAGE_ROOT/msc"
install -m 0755 "$WORKSPACE_ROOT/tools/world-map-proof/target/$RUST_TARGET/release/msc-world-map-proof" \
  "$PACKAGE_ROOT/bedrock-map"
VANTAGE_PLATFORM="macos-aarch64"
if [[ "$PLATFORM_LABEL" == "macos-x86_64" ]]; then
  VANTAGE_PLATFORM="macos-x86_64"
fi
python3 "$WORKSPACE_ROOT/tools/release/stage-vantage.py" \
  --platform "$VANTAGE_PLATFORM" --output-dir "$PACKAGE_ROOT"
printf '%s\n' "$VERSION" > "$PACKAGE_ROOT/MSC-VERSION"
install -m 0755 "$WORKSPACE_ROOT/packaging/macos/install.sh" "$PACKAGE_ROOT/install.sh"
install -m 0755 "$WORKSPACE_ROOT/packaging/macos/uninstall.sh" "$PACKAGE_ROOT/uninstall.sh"
install -m 0644 "$WORKSPACE_ROOT/docs/msc/clients/headless-installation.md" \
  "$PACKAGE_ROOT/HEADLESS-INSTALL.md"
install -m 0644 "$WORKSPACE_ROOT/packaging/macos/com.ctemple.msc.bedrock-helper.plist.in" \
  "$PACKAGE_ROOT/BEDROCK-HELPER-PLIST.in"
if [[ "$INCLUDE_SIDECAR" == true ]]; then
  mkdir -p "$PACKAGE_ROOT/sidecar"
  install -m 0755 "$STAGED_ENGINE/sidecar/BedrockSidecar" "$PACKAGE_ROOT/sidecar/BedrockSidecar"
  install -m 0644 "$STAGED_ENGINE/sidecar/vmlinuz-kata" "$PACKAGE_ROOT/sidecar/vmlinuz-kata"
  install -m 0644 "$STAGED_ENGINE/sidecar/appliance-initramfs.gz" "$PACKAGE_ROOT/sidecar/appliance-initramfs.gz"
fi

rm -f "$ARCHIVE"
tar -czf "$ARCHIVE" -C "$PACKAGE_ROOT" .
printf 'built %s\n' "$ARCHIVE"
