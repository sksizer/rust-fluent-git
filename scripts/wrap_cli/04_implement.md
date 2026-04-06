# Phase 4: Implement Operation Group "{{GROUP}}"

Read the PRD at `docs/prds/fluent-{{NAME}}.md` for the spec.

Read these reference files for the exact patterns to follow:
- `crates/fluent-lima/src/ops/instance.rs` — builder pattern, build_command(), parse output
- `crates/fluent-lima/src/error/instance.rs` — error enum with From<CommandError>
- `crates/fluent-lima/src/types/instance.rs` — serde types
- `crates/fluent-lima/src/sync/lima/ops/instance.rs` — sync run() impls
- `crates/fluent-lima/src/lima/ops/instance.rs` — async run_async() impls

Implement the "{{GROUP}}" operation group for `fluent-{{NAME}}`. Write these files:

1. `crates/fluent-{{NAME}}/src/ops/{{GROUP}}.rs` — builders with `build_command()` and `parse_*_output()` functions
2. `crates/fluent-{{NAME}}/src/error/{{GROUP}}.rs` — domain-specific error enum with manual `From<CommandError>` impl
3. `crates/fluent-{{NAME}}/src/types/{{GROUP}}.rs` — result/info types with serde derives (only if this group returns structured data)
4. `crates/fluent-{{NAME}}/src/sync/{{NAME}}/ops/{{GROUP}}.rs` — sync `run()` impls gated with `#[cfg(feature = "blocking")]`
5. `crates/fluent-{{NAME}}/src/{{NAME}}/ops/{{GROUP}}.rs` — async `run_async()` impls gated with `#[cfg(feature = "tokio")]`

Key rules:
- Builders use `pub(crate) fn new(...)` constructors
- Fluent methods take `mut self` and return `Self`
- Sync run() gated with `#[cfg(feature = "blocking")]`, uses `fluent_core::run_sync`
- Async run_async() gated with `#[cfg(feature = "tokio")]`, uses `fluent_core::run_async`
- Classify stderr patterns into domain-specific error variants
- Use JSON output + serde where the CLI supports it
- Do NOT create mod.rs files — those are assembled in the next phase
