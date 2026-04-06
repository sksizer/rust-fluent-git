# Phase 5: Assemble Modules and Entry Point

Read the PRD at `docs/prds/fluent-{{NAME}}.md` for the full spec.
Read `crates/fluent-lima/src/types/lima.rs` for entry point pattern.
Read `crates/fluent-lima/src/error/mod.rs` for umbrella error pattern.
Read `crates/fluent-lima/src/ops/mod.rs` for ops re-export pattern.

Write all `mod.rs` files and the entry point for `crates/fluent-{{NAME}}/`:

1. `src/ops/mod.rs` — pub mod declarations + re-exports for all operation groups
2. `src/error/mod.rs` — mod declarations + re-exports + `{{ENTRY_TYPE}}Error` umbrella enum with `#[from]` for each group error
3. `src/types/mod.rs` — mod declarations + re-exports for all types
4. `src/sync/{{NAME}}/ops/mod.rs` — mod declarations for sync ops
5. `src/sync/{{NAME}}/mod.rs` — `mod ops;`
6. `src/sync/mod.rs` — `pub mod {{NAME}};`
7. `src/{{NAME}}/ops/mod.rs` — mod declarations for async ops
8. `src/{{NAME}}/mod.rs` — `pub mod ops;`

9. Write the entry point type in `src/types/{{NAME}}.rs`:
   - `#[derive(Debug, Clone, Default)]` struct
   - `impl fluent_core::tool::CliTool` with `program()` returning `"{{TOOL}}"`
   - Factory methods returning each builder type from all operation groups

All pub(crate) constructors must be reachable from the entry point type — no dead code.
