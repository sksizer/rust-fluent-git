---
name: wrap-cli
description: Create a fluent builder-pattern Rust crate wrapping a CLI tool. Use when the user wants to add a new CLI wrapper to the workspace.
---

# Wrap CLI Tool

Create a new `fluent-{name}` crate wrapping a CLI tool with the project's fluent builder-pattern API.

## Arguments

`$ARGUMENTS` should be the CLI tool name (e.g., `docker`, `cargo`, `terraform`). If not provided, ask the user which CLI to wrap.

## Workflow

### Phase 1: Research the CLI

1. Run `{tool} --help` and key subcommand helps to understand the full command surface
2. Check for JSON/structured output support (`--json`, `--format json`, `--output json`, etc.)
3. Test actual output formats where possible (e.g., `{tool} list --json 2>&1 | head`)
4. Identify operation groups (e.g., for `docker`: container, image, volume, network)

### Phase 2: Write the PRD

Create `docs/prds/fluent-{name}.md` with:

- Overview and motivation (why wrap this tool for durable code workflows)
- Scope: in-scope operations (prioritize programmatic/automation use) and out-of-scope (interactive/TUI features)
- Entry point design (stateless like `ClaudeCode`/`Lima` if the tool is general, or bound like `GitHub` if operations target a specific resource)
- Spec checklist with unchecked boxes for every builder, type, and error
- File structure following the project convention
- Design notes on output parsing strategy (JSON via serde vs text parsing)

### Phase 3: Scaffold the Crate

1. Create directory structure:
   ```
   crates/fluent-{name}/src/{ops,error,types,sync/{name}/ops,{name}/ops}
   ```

2. Create `Cargo.toml` following the pattern in existing crates:
   - workspace edition, license, authors
   - features: default=["blocking"], blocking, tokio
   - deps: fluent-core, cmd-spec, thiserror, which, serde, serde_json

3. Create `src/lib.rs` with module declarations

4. Update workspace `Cargo.toml` — add `fluent-{name}` to workspace dependencies

5. Update `crates/fluent-sdlc/Cargo.toml` — add feature flag and optional dep

6. Update `crates/fluent-sdlc/src/lib.rs` — add conditional re-export

### Phase 4: Implement Operations

Use **parallel subagents** for each operation group. Each subagent writes:

- `ops/{group}.rs` — builders with `build_command()` and `parse_*_output()` functions
- `error/{group}.rs` — domain-specific error enum with manual `From<CommandError>` impl
- `types/{group}.rs` — result/info types with serde derives
- `sync/{name}/ops/{group}.rs` �� sync `run()` impls gated with `#[cfg(not(feature = "tokio"))]`
- `{name}/ops/{group}.rs` — async `run()` impls gated with `#[cfg(not(feature = "blocking"))]`

**Subagent instructions must include:**
- Explicit reference files to read for patterns (point to existing ops/pr.rs, error/pr.rs, etc.)
- The entry point type and how to reference it
- Actual CLI output format samples where available
- Instruction to NOT create mod.rs files (assembled after)
- Instruction to run `cargo fmt --all` at end

### Phase 5: Assemble and Fix

1. Write all `mod.rs` files:
   - `ops/mod.rs` — pub mod declarations + re-exports
   - `error/mod.rs` — mod declarations + re-exports + `{Name}Error` umbrella enum
   - `types/mod.rs` — mod declarations + re-exports
   - `sync/{name}/ops/mod.rs`, `sync/{name}/mod.rs`, `sync/mod.rs`
   - `{name}/ops/mod.rs`, `{name}/mod.rs`

2. Write the entry point type in `types/{name}.rs`:
   - Stateless `#[derive(Debug, Clone, Default)]` struct
   - `CliTool` impl with the program name
   - Methods returning each builder type

3. Run `just full-write` then `just full-check` to fix formatting and verify
4. Fix any compilation or clippy errors

### Phase 6: Commit and PR

1. Stage all new/modified files
2. Commit with conventional commit message: `feat: add fluent-{name} crate wrapping {tool} operations`
3. Do NOT include Claude/Anthropic as author
4. Ask user if they want a PR created, and if so, which branch to stack on

## Key Design Rules

- **Builders own their data** — no lifetime parameters for stateless entry points (like ClaudeCode, Lima). Use `&'a EntryPoint` lifetimes only if the entry point holds meaningful state (like GitHub's owner/repo).
- **All types Serialize + Deserialize** — enables workflow checkpointing
- **JSON output preferred** — use `--json`/`--format json` + serde where the CLI supports it
- **Non-interactive by default** — add `--tty=false`, `-y`, `--no-input`, or equivalent flags automatically
- **Domain-specific errors** — classify stderr patterns into typed error variants, with `Command(CommandError)` fallback using manual `From` impl
- **Feature-gated sync/async** — `blocking` (default) and `tokio` features, matching fluent-core
- **No dead code** — ensure all pub(crate) constructors are reachable from the entry point type
