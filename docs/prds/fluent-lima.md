# PRD: fluent-lima

## Overview

A fluent builder-pattern Rust crate (`fluent-lima`) wrapping the Lima CLI (`limactl`) for programmatic management of Linux virtual machines on macOS. Designed as a composable building block for durable code workflows that need isolated Linux environments — sandboxed agent execution, reproducible CI, cross-platform testing.

## Motivation

Lima provides lightweight Linux VMs on macOS with tight host integration (mounts, port forwarding, containerd). Programmatic VM lifecycle management enables workflows that:

- Spin up isolated environments for agent code execution
- Snapshot/restore VM state for reproducible builds
- Copy artifacts between host and guest
- Execute commands in controlled Linux environments

## Scope

### In Scope

**Instance Lifecycle:**
- `create` — from template, URL, or YAML with full config (cpus, memory, disk, mounts, etc.)
- `start` / `stop` / `restart` — lifecycle management
- `delete` — instance removal
- `list` — instance listing with JSON output
- `clone` — instance cloning
- `rename` — instance renaming

**Execution:**
- `shell` — execute commands in guest (non-interactive, captures output)
- `copy` — file transfer between host and guest

**Snapshots:**
- `snapshot create` / `apply` / `delete` / `list` — VM state management

**Disk Management:**
- `disk create` / `list` / `delete` / `resize`

**Instance Protection:**
- `protect` / `unprotect` — prevent accidental deletion

**Diagnostics:**
- `info` — system diagnostic information

### Out of Scope

- Interactive shell sessions (TUI)
- Network management (`limactl network`)
- Template management (`limactl template`)
- sudoers generation
- start-at-login registration
- tunnel management
- YAML validation (use serde directly)
- Watch/events streaming

## Entry Point

```rust
use fluent_lima::Lima;

let lima = Lima::new();

// Create and start a VM
lima.create()
    .name("my-sandbox")
    .template("docker")
    .cpus(4)
    .memory_gib(8)
    .disk_gib(100)
    .mount("/host/path", "/guest/path", true)
    .run()?;

lima.start("my-sandbox").run()?;

// Execute a command
let output = lima.shell("my-sandbox")
    .command("cargo", &["test", "--workspace"])
    .workdir("/workspace")
    .run()?;

// Snapshot for later
lima.snapshot().create("my-sandbox")
    .tag("after-tests")
    .run()?;
```

`Lima` is stateless (like `ClaudeCode`), no lifetime parameters on builders.

## Spec Checklist

### Core Types

- [ ] `Lima` — entry point struct, implements `CliTool` for "limactl"
- [ ] All builders own their data (no lifetime parameters)

### Instance Lifecycle

- [ ] `CreateBuilder` — name, template/url/yaml, cpus, memory, disk, arch, vm_type, mount (multi-call), mount_type, containerd, network, port_forward, plain, rosetta, set (yq expressions)
- [ ] `StartBuilder` — instance name
- [ ] `StopBuilder` — instance name
- [ ] `RestartBuilder` — instance name
- [ ] `DeleteBuilder` — instance name, force flag
- [ ] `ListBuilder` — filter, format (json default), quiet
- [ ] `CloneBuilder` — source instance, destination name
- [ ] `RenameBuilder` — old name, new name

### Execution

- [ ] `ShellBuilder` — instance, command + args, workdir, shell, preserve_env, start (auto-start if stopped)
- [ ] `CopyBuilder` — source, target (prefixed with instance:path for guest), recursive, backend, verbose

### Snapshots

- [ ] `SnapshotBuilder` — entry with `create(instance)`, `apply(instance)`, `delete(instance)`, `list(instance)`
- [ ] `SnapshotCreateBuilder` — instance, tag
- [ ] `SnapshotApplyBuilder` — instance, tag
- [ ] `SnapshotDeleteBuilder` — instance, tag
- [ ] `SnapshotListBuilder` — instance

### Disk Management

- [ ] `DiskBuilder` — entry with `create(name)`, `list()`, `delete(name)`, `resize(name)`
- [ ] `DiskCreateBuilder` — name, size, format
- [ ] `DiskListBuilder`
- [ ] `DiskDeleteBuilder` — name
- [ ] `DiskResizeBuilder` — name, size

### Instance Protection

- [ ] `ProtectBuilder` — instance name
- [ ] `UnprotectBuilder` — instance name

### Diagnostics

- [ ] `InfoBuilder` — system info query

### Output Types (all Serialize + Deserialize)

- [ ] `InstanceInfo` — name, status, vmType, arch, cpus, memory, disk, dir, sshLocalPort, sshAddress, sshConfigFile, protected, hostname
- [ ] `ShellResult` — exit_code, stdout, stderr
- [ ] `SnapshotInfo` — tag and associated metadata
- [ ] `DiskInfo` — name, size, format, instance association
- [ ] `SystemInfo` — parsed from `limactl info`

### Error Types

- [ ] `LimaError` — umbrella enum
- [ ] `InstanceError` — NotFound, AlreadyExists, AlreadyRunning, AlreadyStopped, Protected, Command, Io
- [ ] `ShellError` — NotFound, NotRunning, Command, Io
- [ ] `SnapshotError` — NotFound, TagNotFound, TagAlreadyExists, MustBeStopped, Command, Io
- [ ] `DiskError` — NotFound, AlreadyExists, InUse, Command, Io
- [ ] `CopyError` — NotFound, NotRunning, Command, Io

### Global Flags

All builders should support:
- [ ] `.yes()` — `--tty=false` / `-y` for non-interactive automation
- [ ] Defaults to `--tty=false` since this is programmatic use

### Integration

- [ ] `CliTool` impl for `Lima` (program = "limactl")
- [ ] Workspace dependency in root `Cargo.toml`
- [ ] Feature flag in `fluent-sdlc` (`lima = ["dep:fluent-lima"]`)
- [ ] Sync execution (`blocking` feature)
- [ ] Async execution (`tokio` feature)

### File Structure

```
crates/fluent-lima/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── ops/
    │   ├── mod.rs
    │   ├── instance.rs       # CreateBuilder, StartBuilder, StopBuilder, etc.
    │   ├── shell.rs          # ShellBuilder
    │   ├── copy.rs           # CopyBuilder
    │   ├── snapshot.rs       # SnapshotBuilder + sub-builders
    │   ├── disk.rs           # DiskBuilder + sub-builders
    │   └── info.rs           # InfoBuilder
    ├── error/
    │   ├── mod.rs
    │   ├── instance.rs
    │   ├── shell.rs
    │   ├── copy.rs
    │   ├── snapshot.rs
    │   └── disk.rs
    ├── types/
    │   ├── mod.rs
    │   ├── lima.rs           # Lima entry point
    │   ├── instance.rs       # InstanceInfo
    │   ├── shell.rs          # ShellResult
    │   ├── snapshot.rs       # SnapshotInfo
    │   ├── disk.rs           # DiskInfo
    │   └── info.rs           # SystemInfo
    ├── sync/lima/ops/
    │   └── *.rs
    └── lima/ops/
        └── *.rs
```

## Design Notes

### Non-Interactive by Default

All commands include `--tty=false` automatically since this is a programmatic API. This prevents interactive prompts from blocking workflows.

### JSON Output

`limactl list --json` outputs one JSON object per line (newline-delimited JSON). The parser handles both single-instance and multi-instance responses.

### Shell Execution

`ShellBuilder` is the primary value for workflow use — it enables running arbitrary commands inside the VM and capturing structured output. The builder constructs:
```
limactl shell --tty=false --workdir /path INSTANCE -- command args...
```

### Snapshot Workflow

Snapshots require the instance to be stopped. The error types reflect this constraint so callers can handle it programmatically (stop -> snapshot -> start).
