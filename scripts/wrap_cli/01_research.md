# Phase 1: Research the CLI

Research the CLI tool `{{TOOL}}` and produce a structured analysis.

## Local exploration

1. Run `{{TOOL}} --help` and key subcommand helps to understand the full command surface
2. Check for JSON/structured output support (`--json`, `--format json`, `--output json`, etc.)
3. Test actual output formats where possible (e.g., `{{TOOL}} list --json 2>&1 | head`)
4. Identify operation groups (e.g., for `docker`: container, image, volume, network)

## Web research

5. Search the web for `{{TOOL}}` official documentation to find:
   - The canonical reference for all subcommands and flags (man page, docs site, etc.)
   - Any structured output modes not discoverable from `--help` alone
   - Common automation patterns — how do people script this tool today?
   - Known quirks, non-obvious exit codes, or stderr formats worth classifying into error types

6. If the tool has a REST/gRPC API alternative, note it but prefer the CLI wrapper (that's the project's purpose).

## Output

Write `docs/prds/fluent-{{NAME}}.research.md` with:
- Full help text summary
- Operation groups identified (with subcommands in each)
- JSON output support per subcommand
- Sample JSON outputs where available
- Recommended scope: which operations to wrap (prioritize automation use) and which to skip (interactive/TUI)
- Links to official docs consulted
- Notable stderr patterns / exit codes for error classification
