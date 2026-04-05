# fluent-brew PRD

## Overview

`fluent-brew` wraps the Homebrew (`brew`) package manager CLI with the project's fluent builder-pattern API. This enables durable, type-safe automation of macOS package management — installing formulae/casks, querying package state, managing taps, controlling services, and performing maintenance operations.

## Motivation

Homebrew is the de-facto package manager on macOS and widely used on Linux. Automating brew operations (provisioning machines, managing services, auditing installed packages) currently requires fragile shell scripts. A fluent Rust wrapper provides compile-time safety, structured JSON output parsing, and ergonomic error handling.

## Scope

### In-scope

- **Formula operations**: install, uninstall, reinstall, upgrade, pin, unpin, link, unlink
- **Query operations**: info (JSON v2), search, list (installed), outdated (JSON), deps
- **Tap operations**: tap, untap
- **Service operations**: list, info, start, stop, restart, run, kill
- **Maintenance operations**: update, cleanup, autoremove, doctor

### Out-of-scope

- Interactive/TUI features (edit, irb, sh)
- Developer commands (audit, bottle, bump-*, create, pr-*, test-bot)
- Cask-specific deep operations (beyond --cask flags on shared commands)
- Analytics management

## Entry Point Design

Stateless entry point `Brew` (like `Lima`, `ClaudeCode`) — brew operates on the system package database, not a bound resource.

```rust
let brew = Brew::new();
let installed = brew.list().formulae_only().run()?;
let info = brew.info("ripgrep").run()?;
brew.install("fd").run()?;
brew.services().list().run()?;
```

## Spec Checklist

### Types

- [ ] `Brew` — entry point, implements `CliTool`
- [ ] `FormulaInfo` — JSON v2 formula info
- [ ] `CaskInfo` — JSON v2 cask info
- [ ] `InfoResponse` — wrapper for `--json=v2` output (formulae + casks)
- [ ] `InstalledFormula` — installed formula with version
- [ ] `OutdatedFormula` — outdated formula info
- [ ] `OutdatedCask` — outdated cask info
- [ ] `OutdatedResponse` — wrapper for outdated JSON
- [ ] `ServiceInfo` — service status info
- [ ] `Dependency` — dependency info from deps

### Builders — Formula

- [ ] `InstallBuilder` — `brew install [options] formula|cask`
- [ ] `UninstallBuilder` — `brew uninstall [options] formula|cask`
- [ ] `ReinstallBuilder` — `brew reinstall [options] formula|cask`
- [ ] `UpgradeBuilder` — `brew upgrade [options] [formula|cask]`
- [ ] `PinBuilder` — `brew pin formula`
- [ ] `UnpinBuilder` — `brew unpin formula`
- [ ] `LinkBuilder` — `brew link [options] formula`
- [ ] `UnlinkBuilder` — `brew unlink formula`

### Builders — Query

- [ ] `InfoBuilder` — `brew info --json=v2 formula|cask`
- [ ] `SearchBuilder` — `brew search text`
- [ ] `ListBuilder` — `brew list [options]`
- [ ] `OutdatedBuilder` — `brew outdated --json=v2`
- [ ] `DepsBuilder` — `brew deps [options] formula`

### Builders — Tap

- [ ] `TapBuilder` — `brew tap [user/repo] [URL]`
- [ ] `UntapBuilder` — `brew untap user/repo`

### Builders — Services

- [ ] `ServicesListBuilder` — `brew services list --json`
- [ ] `ServicesInfoBuilder` — `brew services info formula --json`
- [ ] `ServicesStartBuilder` — `brew services start formula`
- [ ] `ServicesStopBuilder` — `brew services stop formula`
- [ ] `ServicesRestartBuilder` — `brew services restart formula`
- [ ] `ServicesRunBuilder` — `brew services run formula`
- [ ] `ServicesKillBuilder` — `brew services kill formula`

### Builders — Maintenance

- [ ] `UpdateBuilder` — `brew update`
- [ ] `CleanupBuilder` — `brew cleanup [options]`
- [ ] `AutoremoveBuilder` — `brew autoremove`
- [ ] `DoctorBuilder` — `brew doctor`

### Errors

- [ ] `FormulaError` — formula operation errors (not found, already installed, etc.)
- [ ] `QueryError` — query/info errors
- [ ] `TapError` — tap errors (already tapped, not tapped, etc.)
- [ ] `ServiceError` — service operation errors
- [ ] `MaintenanceError` — update/cleanup/doctor errors
- [ ] `BrewError` — umbrella error

## File Structure

```
crates/fluent-brew/
  Cargo.toml
  src/
    lib.rs
    ops/
      mod.rs
      formula.rs        # install, uninstall, reinstall, upgrade, pin, unpin, link, unlink
      query.rs          # info, search, list, outdated, deps
      tap.rs            # tap, untap
      services.rs       # services list/info/start/stop/restart/run/kill
      maintenance.rs    # update, cleanup, autoremove, doctor
    error/
      mod.rs            # umbrella BrewError
      formula.rs
      query.rs
      tap.rs
      services.rs
      maintenance.rs
    types/
      mod.rs
      brew.rs           # entry point
      formula.rs        # FormulaInfo, InstalledFormula, etc.
      query.rs          # InfoResponse, OutdatedResponse
      services.rs       # ServiceInfo
    sync/
      mod.rs
      brew/
        mod.rs
        ops/
          mod.rs
          formula.rs
          query.rs
          tap.rs
          services.rs
          maintenance.rs
    brew/
      mod.rs
      ops/
        mod.rs
        formula.rs
        query.rs
        tap.rs
        services.rs
        maintenance.rs
```

## Design Notes

### Output Parsing Strategy

- **`brew info --json=v2`**: Returns rich JSON with `formulae` and `casks` arrays. Parse with serde into `FormulaInfo`/`CaskInfo` types. Use `#[serde(flatten)]` for less-critical nested fields.
- **`brew outdated --json=v2`**: Returns JSON with `formulae` and `casks` arrays showing current/new versions.
- **`brew services list --json`** and **`brew services info --json`**: Returns JSON array of service objects.
- **`brew list`**: Text output (one per line). Use `--versions` for version info.
- **`brew search`**: Text output. Parse lines.
- **`brew deps`**: Text output. Parse lines.
- **`brew doctor`**: Text output. Capture as string — success = no issues.
- **`brew update`/`brew cleanup`**: Text output. Return `()` on success.

### Non-interactive Defaults

- No brew commands require TTY by default, but add `HOMEBREW_NO_AUTO_UPDATE=1` environment variable to prevent automatic update checks during install/upgrade operations.
