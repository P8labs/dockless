#!/usr/bin/env bash

set -euo pipefail

GITHUB_REPO="p8labs/dockless"
BINARY_NAME="dockless"
INSTALL_DIR="/usr/local/bin"
BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

TMP_DIR=""

log() {
  echo -e "\033[1;34m[Dockless]\033[0m $1"
}

error() {
  echo -e "\033[1;31mError:\033[0m $1"
  exit 1
}

cleanup() {
  if [ -n "${TMP_DIR:-}" ] && [ -d "$TMP_DIR" ]; then
    rm -rf "$TMP_DIR"
  fi
}

trap 'error "Upgrade failed at line $LINENO. Aborting."' ERR
trap cleanup EXIT

require_root() {
  if [ "$EUID" -ne 0 ]; then
    error "Please run as root (use sudo)."
  fi
}

check_installed() {
  [ -f "$BINARY_PATH" ] || error "Dockless not found at $BINARY_PATH"
}

check_dependencies() {
  for cmd in curl sha256sum systemctl; do
    command -v $cmd >/dev/null 2>&1 || error "Missing dependency: $cmd"
  done
}

detect_arch() {
  ARCH=$(uname -m)
  case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64 | arm64) ARCH="aarch64" ;;
    armv7l) ARCH="armv7" ;;
    *) error "Unsupported architecture: $ARCH" ;;
  esac
}

get_current_version() {
  CURRENT_VERSION=$("$BINARY_PATH" --version 2>/dev/null | awk '{print $2}' || true)

  if [ -z "${CURRENT_VERSION:-}" ]; then
    log "Could not determine current version."
    CURRENT_VERSION="unknown"
  fi

  log "Current version: $CURRENT_VERSION"
}

fetch_latest_version() {
  log "Fetching latest release..."

  LATEST_VERSION=$(curl -fsSL "https://api.github.com/repos/$GITHUB_REPO/releases/latest" \
    | grep '"tag_name":' \
    | sed -E 's/.*"([^"]+)".*/\1/')

  [ -n "$LATEST_VERSION" ] || error "Could not determine latest version."

  log "Latest version: $LATEST_VERSION"
}

compare_versions() {
  if [ "$CURRENT_VERSION" = "$LATEST_VERSION" ]; then
    log "Dockless is already up to date."
    exit 0
  fi
}

download_and_verify() {
  ASSET="${BINARY_NAME}-${LATEST_VERSION}-linux-${ARCH}"
  BASE_URL="https://github.com/${GITHUB_REPO}/releases/download/${LATEST_VERSION}"
  BINARY_URL="${BASE_URL}/${ASSET}"
  CHECKSUM_URL="${BASE_URL}/checksums.txt"

  log "Downloading new binary..."
  TMP_DIR=$(mktemp -d)

  curl -fL "$BINARY_URL" -o "$TMP_DIR/$ASSET" || error "Binary download failed."

  log "Attempting checksum verification..."

  if curl -fsL "$CHECKSUM_URL" -o "$TMP_DIR/checksums.txt"; then
    cd "$TMP_DIR"

    EXPECTED_SUM=$(grep "$ASSET" checksums.txt | awk '{print $1}' || true)

    if [ -n "$EXPECTED_SUM" ]; then
      ACTUAL_SUM=$(sha256sum "$ASSET" | awk '{print $1}')

      if [ "$EXPECTED_SUM" != "$ACTUAL_SUM" ]; then
        error "Checksum mismatch. Aborting upgrade."
      fi

      log "Checksum verified."
    else
      log "Checksum entry not found. Skipping verification."
    fi
  else
    log "No checksums.txt found. Skipping verification."
  fi

  mv "$TMP_DIR/$ASSET" "$BINARY_PATH"
  chmod +x "$BINARY_PATH"

  log "Binary replaced successfully."
}

restart_service() {
  log "Restarting Dockless..."

  if systemctl is-active --quiet dockless; then
    systemctl restart dockless
  else
    systemctl start dockless
  fi

  if systemctl is-active --quiet dockless; then
    log "Dockless is running."
  else
    error "Dockless failed to start after upgrade."
  fi
}


require_root
check_dependencies
check_installed
detect_arch
get_current_version
fetch_latest_version
compare_versions
download_and_verify
restart_service

log "Upgrade complete."