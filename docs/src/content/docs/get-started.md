---
title: Get Started
description: Install, configure, and run Dockless as a system service.
---

# Get Started

Dockless runs as a persistent system service. Once installed, it starts automatically on boot and exposes a local web dashboard for managing services.

This guide covers installation, verification, upgrade, and removal.

---

## System Requirements

- Linux-based distribution
- systemd available
- curl and tar installed
- Root privileges for installation

Dockless currently requires systemd.

---

## Install Dockless

Install using the official script:

```bash
curl -fsSL https://dockless.p8labs.tech/install.sh | sudo bash
```

The installer will:

- Detect your system architecture
- Download the latest release from GitHub
- Install the binary to `/usr/local/bin/dockless`
- Create `/etc/dockless/`
- Generate a default configuration
- Create and enable a systemd service
- Start Dockless automatically

---

## Default Configuration

The installer creates:

```text
/etc/dockless/config.toml
```

With the following defaults:

```toml
data_dir = "/etc/dockless/data"
listen_port = 3080
```

You may edit this file at any time.

After modifying configuration, restart the service:

```bash
sudo systemctl restart dockless
```

---

## Verify Installation

Check service status:

```bash
systemctl status dockless
```

If running correctly, you should see:

```text
active (running)
```

View logs:

```bash
journalctl -u dockless -f
```

---

## Access the Web Dashboard

After installation, the installer prints available access URLs.

Typically:

```text
http://<your-local-ip>:3080
```

If mDNS is available:

```text
http://<hostname>.local:3080
```

You can also access locally:

```text
http://localhost:3080
```

The dashboard allows you to:

- Register services
- Monitor runtime state
- Restart services
- View logs

---

## Upgrade Dockless

To upgrade to the latest version:

```bash
curl -fsSL https://dockless.p8labs.tech/upgrade.sh | sudo bash
```

The upgrade script will:

- Detect the current installed version
- Fetch the latest release
- Replace the binary safely
- Restart the systemd service

Configuration and data are not modified.

---

## Uninstall

To remove Dockless:

```bash
sudo systemctl stop dockless
sudo systemctl disable dockless
sudo rm /usr/local/bin/dockless
sudo rm -rf /etc/dockless
sudo rm /etc/systemd/system/dockless.service
sudo systemctl daemon-reload
```

---
