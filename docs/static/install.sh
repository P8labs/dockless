#!/usr/bin/env bash

set -euo pipefail

GITHUB_REPO="p8labs/dockless"
BINARY_NAME="dockless"
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/dockless"
CONFIG_FILE="$CONFIG_DIR/config.toml"
DATA_DIR="$CONFIG_DIR/data"
SERVICE_FILE="/etc/systemd/system/dockless.service"

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

trap 'error "Installation failed at line $LINENO. Aborting."' ERR
trap cleanup EXIT

require_root() {
  if [ "$EUID" -ne 0 ]; then
    error "Please run as root (use sudo)."
  fi
}

check_dependencies() {
  for cmd in curl systemctl sha256sum ss; do
    command -v $cmd >/dev/null 2>&1 || error "Required dependency missing: $cmd"
  done
}

detect_os() {
  OS=$(uname -s)
  [ "$OS" = "Linux" ] || error "Unsupported OS: $OS"
}


detect_arch() {
  ARCH=$(uname -m)
  case "$ARCH" in
    x86_64) ARCH="amd64" ;;
    aarch64 | arm64) ARCH="arm64" ;;
    armv7l) ARCH="armv7" ;;
    *) error "Unsupported architecture: $ARCH" ;;
  esac
}

check_systemd() {
  command -v systemctl >/dev/null 2>&1 || error "systemd not detected."
}

check_port_available() {
  PORT=3080
  if ss -tuln | grep -q ":$PORT "; then
    error "Port $PORT is already in use. Please free it before installing."
  fi
}

fetch_latest_release() {
  log "Fetching latest release..."

  VERSION=$(curl -fsSL "https://api.github.com/repos/$GITHUB_REPO/releases/latest" \
    | grep '"tag_name":' \
    | sed -E 's/.*"([^"]+)".*/\1/')

  [ -n "$VERSION" ] || error "Could not determine latest version."

  log "Latest version: $VERSION"
}

download_and_verify() {
  ASSET="${BINARY_NAME}-${VERSION}-linux-${ARCH}"
  BASE_URL="https://github.com/${GITHUB_REPO}/releases/download/${VERSION}"
  BINARY_URL="${BASE_URL}/${ASSET}"
  CHECKSUM_URL="${BASE_URL}/${ASSET}.sha256"

  log "Downloading binary..."
  TMP_DIR=$(mktemp -d)

  curl -fL "$BINARY_URL" -o "$TMP_DIR/$ASSET" || error "Binary download failed."

  log "Attempting checksum verification..."

  if curl -fsL "$CHECKSUM_URL" -o "$TMP_DIR/$ASSET.sha256"; then
    cd "$TMP_DIR"

    EXPECTED_SUM=$(awk '{print $1}' "$ASSET.sha256")
    ACTUAL_SUM=$(sha256sum "$ASSET" | awk '{print $1}')

    if [ "$EXPECTED_SUM" != "$ACTUAL_SUM" ]; then
      error "Checksum mismatch. Aborting installation."
    fi

    log "Checksum verified."
  else
    log "No checksum file found. Skipping verification."
  fi

  mv "$TMP_DIR/$ASSET" "$INSTALL_DIR/$BINARY_NAME"
  chmod +x "$INSTALL_DIR/$BINARY_NAME"

  log "Binary installed to $INSTALL_DIR/$BINARY_NAME"
}

setup_config() {
  log "Setting up configuration..."

  mkdir -p "$CONFIG_DIR"
  mkdir -p "$DATA_DIR"

  if [ ! -f "$CONFIG_FILE" ]; then
    cat > "$CONFIG_FILE" <<EOF
data_dir = "$DATA_DIR"
listen_port = 3080
EOF
    log "Created default config."
  else
    log "Existing config detected. Skipping."
  fi
}

setup_systemd() {
  log "Creating systemd service..."

  cat > "$SERVICE_FILE" <<EOF
[Unit]
Description=Dockless Runtime
After=network.target

[Service]
ExecStart=$INSTALL_DIR/$BINARY_NAME
Restart=always
RestartSec=3
User=root
WorkingDirectory=$CONFIG_DIR

[Install]
WantedBy=multi-user.target
EOF

  systemctl daemon-reload
  systemctl enable dockless
  systemctl restart dockless
}

verify_install() {
  log "Verifying service..."

  if systemctl is-active --quiet dockless; then
    log "Dockless is running."
  else
    error "Dockless failed to start. Check: journalctl -u dockless"
  fi
}

detect_access_info() {
  PORT=$(grep listen_port "$CONFIG_FILE" | awk -F'=' '{print $2}' | tr -d ' ')

  IP=$(hostname -I 2>/dev/null | awk '{print $1}')
  [ -z "$IP" ] && IP=$(ip route get 1 | awk '{print $7; exit}')
  [ -z "$IP" ] && IP="localhost"

  MDNS_HOST=""
  if command -v avahi-resolve >/dev/null 2>&1; then
    HOSTNAME=$(hostname)
    MDNS_HOST="${HOSTNAME}.local"
  fi

  log ""
  log "Access Dockless at:"
  log "  http://$IP:$PORT"

  if [ -n "$MDNS_HOST" ]; then
    log "  http://$MDNS_HOST:$PORT"
  fi
}

require_root
check_dependencies
detect_os
detect_arch
check_systemd
check_port_available
fetch_latest_release
download_and_verify
setup_config
setup_systemd
verify_install

log "Installation complete."
detect_access_info