# Phase 3: Scaffold the Crate

Read the PRD at `docs/prds/fluent-{{NAME}}.md` for the spec.
Read `crates/fluent-lima/Cargo.toml` and `crates/fluent-lima/src/lib.rs` for the pattern.

1. Create directory structure:
   ```
   crates/fluent-{{NAME}}/src/{ops,error,types,sync/{{NAME}}/ops,{{NAME}}/ops}
   ```

2. Create `Cargo.toml` following the pattern in existing crates:
   - workspace edition, license, authors
   - features: default=["blocking"], blocking, tokio
   - deps: fluent-core, cmd-spec, thiserror, which, serde, serde_json

3. Create `src/lib.rs` with module declarations

4. Update workspace `Cargo.toml` — add `fluent-{{NAME}}` to workspace dependencies

5. Update `crates/fluent-sdlc/Cargo.toml` — add feature flag and optional dep

6. Update `crates/fluent-sdlc/src/lib.rs` — add conditional re-export

Do NOT implement any operations yet. Just create the skeleton.
