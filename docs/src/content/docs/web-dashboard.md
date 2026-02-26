---
title: Web Dashboard
description: Manage and monitor Dockless through the local web interface.
---

# Web Dashboard

Dockless provides a local web dashboard for managing services and monitoring runtime state.

The dashboard runs on the port defined in your configuration:

```toml
listen_port = 3080
```

By default, access it at:

```
http://<your-ip>:3080
```

If running locally:

```
[http://localhost:3080](http://localhost:3080)

```

If mDNS is available:

```

http://<hostname>.local:3080

```

---

## Purpose

The web dashboard is the primary management interface for Dockless.

It allows you to:

- Register new services
- View running services
- Restart or stop services
- Monitor runtime status
- Inspect logs

The dashboard is designed for local or controlled environments.

---

## Service Overview

The main view displays:

- Registered services
- Current process status
- Uptime information
- Port assignments
- Restart state

Each service is supervised directly by Dockless. If a service exits unexpectedly, Dockless applies its restart policy.

---

## Registering a Service

When adding a new service through the dashboard, you typically define:

- Service name
- Path to the binary
- Working directory
- Port (if required)
- Restart behavior

Dockless launches the binary as a host process and begins supervising it.

There is no container isolation. Services run directly on the host system.

---

## Logs

The dashboard provides access to runtime logs for managed services.

For deeper debugging, use systemd logs:

```
journalctl -u dockless -f
```

---

## Restarting Services

Services can be restarted individually from the dashboard.

Restarting a service does not restart Dockless itself.

To restart the entire runtime:

```bash
sudo systemctl restart dockless
```

---

## Security Considerations

The web dashboard does not implement built-in authentication by default.

If exposing Dockless beyond localhost:

- Use a reverse proxy
- Enable HTTPS
- Restrict access via firewall rules
- Avoid exposing management ports directly to the public internet

Dockless is intended for trusted environments.

---

## Network Binding

Dockless binds to the host network stack directly.

It does not create virtual networking layers or overlay networks.

Port conflicts must be avoided at the system level.

If Dockless fails to start due to port conflicts, check:

```bash
ss -tuln | grep <port>
```

---

## Future Improvements

Planned improvements may include:

- Authentication support
- Access control configuration
- Improved logging visibility
- Runtime metrics

The dashboard will evolve while keeping the runtime minimal.
