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
  if [ ! -f "$BINARY_PATH" ]; then
    error "Dockless is not installed at $BINARY_PATH"
  fi
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

download_new_binary() {
  ASSET="${BINARY_NAME}-${LATEST_VERSION}-linux-${ARCH}.tar.gz"
  URL="https://github.com/${GITHUB_REPO}/releases/download/${LATEST_VERSION}/${ASSET}"

  log "Downloading $ASSET..."

  TMP_DIR=$(mktemp -d)

  curl -fL "$URL" -o "$TMP_DIR/$ASSET" || error "Download failed."
  tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR" || error "Extraction failed."

  [ -f "$TMP_DIR/$BINARY_NAME" ] || error "Binary not found in archive."

  mv "$TMP_DIR/$BINARY_NAME" "$BINARY_PATH"
  chmod +x "$BINARY_PATH"

  log "Binary upgraded successfully."
}

restart_service() {
  if systemctl is-active --quiet dockless; then
    log "Restarting Dockless..."
    systemctl restart dockless
  else
    log "Dockless service not running. Starting..."
    systemctl start dockless
  fi

  if systemctl is-active --quiet dockless; then
    log "Dockless is running."
  else
    error "Dockless failed to start after upgrade."
  fi
}


require_root
check_installed
detect_arch
get_current_version
fetch_latest_version
compare_versions
download_new_binary
restart_service

log "Upgrade complete."