---
title: Introduction
description: Learn what Dockless is, why we built it, how to set it up, and understand its underlying architecture.
---

# Introduction

## What is Dockless

Dockless is a minimal, lightweight system service designed to manage applications directly on your infrastructure. Rather than relying on heavy orchestration systems or complex container abstractions, Dockless runs close to the host OS. This approach ensures maximal performance, predictable resource usage, and simpler debugging for production applications.

By operating as a fundamental system component, Dockless acts as a straightforward bridge between your application binaries or runtimes and the underlying operating system.

## Why Dockless

Traditional orchestration platforms often introduce significant operational overhead. They require you to understand new networking models, volume management abstractions, and complex deployment manifests. Dockless exists for environments where these features are unnecessary.

- **Minimal Overhead**: Focuses only on running and keeping your services alive.
- **Direct Access**: Services managed by Dockless have direct interaction with host network interfaces and disk, bypassing virtualized bridge networks.
- **Native Integration**: Seamlessly integrates with standard system components like systemd and standard logging daemons.

## How It Runs

Dockless is installed and operates as a background daemon, running as a standard system service. It typically integrates with `systemd` to ensure it starts automatically at boot and manages its lifecycle correctly.

Here is an example `systemd` service configuration snippet for running Dockless automatically on startup:

```ini
[Unit]
Description=Dockless System Service
After=network.target

[Service]
ExecStart=/usr/local/bin/dockless start
Restart=always
RestartSec=3
User=root
Group=root
Environment="DOCKLESS_ENV=production"
Environment="DOCKLESS_PORT=8080"

[Install]
WantedBy=multi-user.target
```

## Architecture Overview

Dockless is composed of two primary layers: the core execution engine and the local web dashboard.

The **core execution engine** runs headless, monitoring your managed tasks, handling logs, and ensuring that everything stays operational according to your configuration.

The **local web dashboard** is a built-in graphical interface that provides real-time visibility into the health and metrics of your applications. Once Dockless is running, you can securely access this dashboard using your configured IP address and port from your web browser. For example, if configured locally on port `8080`, simply navigate to `http://127.0.0.1:8080` to view the service status and logs.
