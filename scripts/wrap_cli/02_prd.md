# Phase 2: Write the PRD

Read `docs/prds/fluent-{{NAME}}.research.md` for context on the CLI tool.
Read an existing PRD for reference: `docs/prds/fluent-lima.md`.

Create `docs/prds/fluent-{{NAME}}.md` with:

- Overview and motivation (why wrap this tool for durable code workflows)
- Scope: in-scope operations (prioritize programmatic/automation use) and out-of-scope (interactive/TUI features)
- Entry point design (stateless like `ClaudeCode`/`Lima` if the tool is general, or bound like `GitHub` if operations target a specific resource)
- Spec checklist with unchecked boxes for every builder, type, and error
- File structure following the project convention
- Design notes on output parsing strategy (JSON via serde vs text parsing)

The PRD must be complete enough that a developer could implement the crate from it alone.
