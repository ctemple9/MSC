#!/usr/bin/env bash
set -Eeuo pipefail

PATH_DIR="/usr/local/bin"
PATH_LINK="$PATH_DIR/msc"
INSTALL_BASE="/usr/local/lib/msc2"

fail() {
  printf 'MSC macOS headless uninstaller: %s\n' "$1" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage: uninstall.sh

Remove the macOS headless MSC command and its launchd engine service.
Managed server data remains in place.
USAGE
}

while (($# > 0)); do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      fail "unknown argument: $1"
      ;;
  esac
done

[[ "$(uname -s)" == "Darwin" ]] || fail "this package only uninstalls on macOS"
command -v sudo >/dev/null 2>&1 || ((EUID == 0)) || fail "sudo is required for uninstallation"
command -v readlink >/dev/null 2>&1 || fail "readlink is required"

if ((EUID != 0)); then
  exec sudo "$0" "$@"
fi

case "$(uname -m)" in
  x86_64) ARCHITECTURE="x86_64" ;;
  arm64|aarch64) ARCHITECTURE="aarch64" ;;
  *) fail "unsupported macOS architecture: $(uname -m)" ;;
esac

ARCH_ROOT="$INSTALL_BASE/$ARCHITECTURE"
PATH_LINK_STATE="not present"
ENGINE_PLIST="/Library/LaunchDaemons/com.ctemple.msc2.agent.plist"
if [[ -f "$ENGINE_PLIST" ]]; then
  SERVICE_BINARY="$(plutil -extract ProgramArguments.0 raw -o - "$ENGINE_PLIST" 2>/dev/null)" || \
    fail "could not inspect the installed engine service"
  [[ "$SERVICE_BINARY" == "$ARCH_ROOT/"*/msc ]] || fail \
    "the installed engine service belongs to another MSC installation"
  /bin/launchctl bootout system "$ENGINE_PLIST"
  rm -f "$ENGINE_PLIST"
fi

# Only remove a command link when its target is inside MSC's architecture root
# and that exact version directory carries MSC's ownership marker.
if [[ -L "$PATH_LINK" ]]; then
  LINK_TARGET="$(readlink "$PATH_LINK")"
  ARCH_PREFIX="$ARCH_ROOT/"
  if [[ "$LINK_TARGET" == "$ARCH_PREFIX"*/msc ]]; then
    VERSION_ROOT="$(dirname -- "$LINK_TARGET")"
    if [[ -f "$VERSION_ROOT/.msc2-owned" ]] &&
       [[ "$(sed -n '1p' "$VERSION_ROOT/.msc2-owned")" == "msc2-headless-archive" ]]; then
      rm -f "$PATH_LINK"
      PATH_LINK_STATE="removed"
    else
      PATH_LINK_STATE="left unchanged (the target is not an MSC-owned installation)"
    fi
  else
    PATH_LINK_STATE="left unchanged (the target is outside the MSC installation root)"
  fi
elif [[ -e "$PATH_LINK" ]]; then
  PATH_LINK_STATE="left unchanged (the path is not an MSC-owned symlink)"
fi

# Upgrades retain old version directories for rollback. Remove only version
# directories carrying MSC's marker, and leave any unmarked user content alone.
if [[ -d "$ARCH_ROOT" && ! -L "$ARCH_ROOT" ]]; then
  for VERSION_ROOT in "$ARCH_ROOT"/*; do
    [[ -d "$VERSION_ROOT" && ! -L "$VERSION_ROOT" ]] || continue
    if [[ -f "$VERSION_ROOT/.msc2-owned" ]] &&
       [[ "$(sed -n '1p' "$VERSION_ROOT/.msc2-owned")" == "msc2-headless-archive" ]]; then
      rm -rf "$VERSION_ROOT"
    fi
  done
fi

rmdir "$ARCH_ROOT" >/dev/null 2>&1 || true
rmdir "$INSTALL_BASE" >/dev/null 2>&1 || true

cat <<MESSAGE
MSC macOS headless command removed.

The MSC-owned command link at $PATH_LINK was $PATH_LINK_STATE.
Managed server data was retained.
MESSAGE
