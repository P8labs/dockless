---
title: Runtime Model
description: Understand how Dockless runs as a system service and manages applications.
---

# Runtime Model

Dockless is designed as a single-node runtime that runs as a persistent system service.

It is not a container engine and does not provide multi-node orchestration (yet). Its purpose is to supervise and manage statically compiled services on constrained hardware in a predictable way.

---

## Process Architecture

Dockless runs as a background service managed by systemd.

After installation, systemd starts Dockless at boot:

```text
/etc/systemd/system/dockless.service
```

The service launches the Dockless binary:

```ini
ExecStart=/usr/local/bin/dockless
WorkingDirectory=/etc/dockless
```

Once running, Dockless:

- Loads configuration from `config.toml`
- Initializes runtime state
- Starts listening on the configured port
- Prepares service supervision logic

Dockless itself remains active until stopped by systemd.

---

## Single-Node Design

Dockless operates on a single machine. It does not:

- Coordinate with other nodes
- Replicate state
- Provide distributed scheduling

This design keeps the runtime simple and predictable.

It is optimized for:

- Raspberry Pi
- ARM devices
- Low-memory systems
- Self-hosted environments

---

## Service Supervision

Dockless supervises managed services directly as host processes.

It does not use container isolation or layered filesystems.

When a service is registered:

- Dockless launches the binary
- Tracks its process ID
- Monitors exit state
- Restarts it if configured to do so

This model reduces overhead compared to container-based runtimes.

---

## Networking Model

Dockless exposes a local web dashboard on the configured `listen_port`.

```toml
listen_port = 3080
```

The runtime binds to the host network stack directly.

There is no internal virtual network layer. Services run on the host and bind to host ports.

Port management and conflict avoidance are handled at the runtime level.

---

## Data Storage

Dockless stores runtime data in:

```text
/etc/dockless/data
```

This includes:

- Runtime metadata
- Service definitions
- State files
- Logs (if enabled)

The `data_dir` is configurable.

---

## Failure Handling

Dockless relies on two layers of resilience:

1. Internal service supervision
2. systemd restart policy

If Dockless crashes:

- systemd restarts it automatically

If a managed service crashes:

- Dockless detects termination
- Applies restart policy if configured

This layered model ensures recovery at both runtime and service levels.

---

## Boot Lifecycle

At system boot:

1. systemd starts Dockless
2. Dockless loads configuration
3. Runtime state is initialized
4. Services are restored and started

This ensures persistent behavior across reboots.

---

## Web Dashboard

Dockless exposes a local web interface at:

```text
http://<your-ip>:<listen_port>
```

The dashboard allows:

- Service registration
- Runtime monitoring
- Log inspection
- Restart operations

It is intended for local or controlled access.

---

## Design Philosophy

Dockless prioritizes:

- Simplicity
- Low overhead
- Host-level transparency
- Predictable behavior
- Minimal abstraction

It avoids complex orchestration layers intentionally.

The runtime model is intentionally narrow in scope.
