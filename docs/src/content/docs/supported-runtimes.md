---
title: Supported Runtimes
description: Understand which application types Dockless is designed to run.
---

# Supported Runtimes

Dockless is designed primarily for statically compiled applications running as host processes.

It is optimized for low-resource edge systems where minimal overhead and predictable behavior are required.

---

## Primary Target: Static Binaries

Dockless works best with statically compiled binaries.

Examples:

- Go applications compiled with static linking
- Rust binaries
- C or C++ applications
- Zig binaries
- Other compiled executables that do not require external runtime dependencies

These applications:

- Run directly on the host
- Require no interpreter
- Have minimal runtime overhead
- Are well suited for edge hardware such as Raspberry Pi

This is the recommended deployment model.

---

## Dynamic Applications

Dynamically linked binaries can also run under Dockless, provided that:

- All required shared libraries are present on the system
- The host environment satisfies runtime dependencies

Dockless does not manage dependency resolution. It supervises processes only.

If the host system is properly configured, dynamic applications may run without issue.

---

## Embedded Web Servers

Web applications embedded directly inside a compiled binary are fully supported.

Examples:

- A Go application serving HTTP internally
- A Rust binary exposing a web API
- A C++ service exposing REST endpoints

This model is ideal for Dockless.

The application binds to a host port and Dockless supervises the process.

---

## Node.js and Python

Dockless does not officially target Node.js or Python-based applications.

While it is technically possible to run:

```bash
node app.js
python app.py
```

Dockless does not:

- Install interpreters
- Manage virtual environments
- Resolve language-level dependencies

These runtimes introduce additional overhead that conflicts with Dockless’ edge-focused design goals.

For constrained systems, statically compiled binaries are strongly recommended.

---

## Containers

Dockless is not a container runtime.

It does not:

- Provide namespace isolation
- Provide cgroups management
- Offer layered filesystems
- Replace Docker or Kubernetes

Dockless supervises host-level processes directly.

---

## Recommended Deployment Model

For best results:

1. Compile your application as a static binary.
2. Ensure it can run independently on the host.
3. Register it through the Dockless dashboard.
4. Let Dockless supervise and restart it if necessary.

This approach provides:

- Minimal memory footprint
- Faster startup time
- Reduced operational complexity
- Predictable behavior on edge hardware

---

## Design Intent

Dockless prioritizes simplicity and low overhead.

It is not intended to be a general-purpose runtime for every language ecosystem.

It is intended to run efficient, standalone services in controlled environments.
