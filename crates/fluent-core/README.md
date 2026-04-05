# fluent-core

Shared execution infrastructure for fluent CLI tool wrappers.

## What it provides

- `run_sync()` / `run_async()` — execute a `ShellCommand` and get `std::process::Output`
- `stdout_string()` / `stderr_string()` — extract trimmed strings from output
- `CommandError` — generic fallback for uncategorized CLI failures
- `CliTool` trait — discover CLI binaries on PATH
- Re-exports `cmd_spec::ShellCommand`

## Features

- `blocking` (default) — enables `run_sync`
- `tokio` — enables `run_async`

Tool crates like `fluent-git` depend on this. You typically don't depend on it directly.
