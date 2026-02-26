---
title: Configuration
description: Configure Dockless runtime settings and behavior.
---

# Configuration

Dockless is configured using a single TOML file.

By default, the configuration file is located at:

```

/etc/dockless/config.toml

```

This file is created automatically during installation.

---

## Configuration File Format

Dockless uses the TOML format for configuration. A minimal configuration looks like:

```toml
data_dir = "/etc/dockless/data"
listen_port = 3080
```

After modifying the configuration, restart Dockless:

```bash
sudo systemctl restart dockless
```

---

## Configuration Options

### `data_dir`

```toml
data_dir = "/etc/dockless/data"
```

Defines the directory where Dockless stores:

- Runtime state
- Service metadata
- Logs (if applicable)
- Internal data files

This directory must be writable by the Dockless process.

If you change this path, ensure the directory exists and has proper permissions.

---

### `listen_port`

```toml
listen_port = 3080
```

Defines the port where the Dockless web dashboard will listen.

After changing the port, restart the service and access:

```
http://<your-ip>:<new-port>
```

Make sure the port is not already in use by another service.

---

## Changing the Data Directory

To move runtime data to another location:

1. Stop Dockless:

```bash
sudo systemctl stop dockless
```

2. Move existing data:

```bash
sudo mv /etc/dockless/data /new/location
```

3. Update `config.toml`:

```toml
data_dir = "/new/location"
```

4. Restart Dockless:

```bash
sudo systemctl start dockless
```

---

## Changing the Listen Port

Edit:

```
/etc/dockless/config.toml
```

Example:

```toml
listen_port = 9090
```

Restart the service:

```bash
sudo systemctl restart dockless
```

Then access:

```
http://<your-ip>:9090
```

---

## Production Considerations

If exposing Dockless beyond localhost:

- Use a reverse proxy such as Nginx
- Enable HTTPS
- Restrict access via firewall rules
- Avoid exposing management ports directly to the internet

Dockless is designed for controlled environments.

---

## Reloading Configuration

Dockless currently requires a service restart to apply configuration changes:

```bash
sudo systemctl restart dockless
```

Hot reload support may be added in future versions.

---

## Troubleshooting Configuration

If Dockless fails to start after editing configuration:

Check logs:

```bash
journalctl -u dockless -f
```

Common causes:

- Invalid TOML syntax
- Port already in use
- Directory permissions incorrect
