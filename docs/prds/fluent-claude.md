# PRD: fluent-claude

## Overview

A fluent builder-pattern Rust crate (`fluent-claude`) wrapping the Claude Code CLI (`claude`) for programmatic, non-interactive use. Designed as a composable building block for durable code workflows — not a workflow engine itself, but a reliable, serializable interface to Claude Code's headless capabilities.

## Motivation

Claude Code's `--print` mode enables non-interactive execution with structured JSON output, making it suitable for programmatic orchestration. However, constructing the right CLI invocations with correct flag combinations, parsing streaming/batch JSON responses, and handling error states requires significant boilerplate. This crate provides:

- Type-safe builder API for constructing invocations
- Serde-based input/output types for serialization across workflow boundaries
- Structured error classification for programmatic error handling
- Support for both sync and async execution (matching existing crate conventions)

## Scope

Wraps the `claude` CLI's **non-interactive** capabilities only. Interactive/TUI mode is out of scope.

### In Scope

- Prompt execution (`claude -p`)
- Session management (continue, resume, fork)
- Model and effort configuration
- Tool access control (allowed/disallowed tools)
- Output format control (text, json, stream-json)
- Structured output with JSON schema validation
- System prompt customization
- MCP server configuration
- Budget limits
- Permission modes
- Working directory and additional directory configuration
- Auth status checking

### Out of Scope

- Interactive TUI session management
- Plugin management (`claude plugin`)
- MCP server CRUD (`claude mcp add/remove`)
- Auto-updater (`claude update`, `claude install`)
- Doctor/diagnostics (`claude doctor`)
- Stream-json input mode (complex bidirectional streaming)

## Entry Point

```rust
use fluent_claude::ClaudeCode;

let claude = ClaudeCode::new();

// Simple prompt execution
let result = claude
    .prompt("Fix the failing test in src/lib.rs")
    .model("sonnet")
    .run()?;

// Returns structured result with response text, token usage, etc.
println!("{}", result.response);
```

Unlike `fluent-gh` (which binds to `owner/repo`) or `fluent-git` (which binds to a path), `ClaudeCode` is stateless by default — configuration is per-invocation. Working directory can be set per-call.

## Spec Checklist

### Core Types

- [ ] `ClaudeCode` — entry point struct, optional default config (model, working dir)
- [ ] `PromptBuilder` — primary builder for `claude -p` invocations
- [ ] `SessionResumeBuilder` — builder for `claude --resume` / `--continue`

### PromptBuilder Configuration

- [ ] `.prompt(text)` — the prompt string (positional arg)
- [ ] `.model(model)` — `--model` flag (e.g., "sonnet", "opus", "claude-sonnet-4-6")
- [ ] `.effort(level)` — `--effort` flag (low, medium, high, max)
- [ ] `.system_prompt(text)` — `--system-prompt` override
- [ ] `.append_system_prompt(text)` — `--append-system-prompt`
- [ ] `.output_format(format)` — `--output-format` (text, json, stream-json)
- [ ] `.json_schema(schema)` — `--json-schema` for structured output validation
- [ ] `.max_budget(usd)` — `--max-budget-usd` spending limit
- [ ] `.permission_mode(mode)` — `--permission-mode` (default, auto, plan, acceptEdits, bypassPermissions)
- [ ] `.allowed_tools(tools)` — `--allowedTools` whitelist
- [ ] `.disallowed_tools(tools)` — `--disallowedTools` blacklist
- [ ] `.tools(tools)` — `--tools` built-in tool set override
- [ ] `.add_dir(path)` — `--add-dir` additional directory access
- [ ] `.working_dir(path)` — sets CWD for the subprocess
- [ ] `.mcp_config(path_or_json)` — `--mcp-config`
- [ ] `.session_id(uuid)` — `--session-id` for deterministic session IDs
- [ ] `.name(name)` — `--name` display name for the session
- [ ] `.bare()` — `--bare` minimal mode
- [ ] `.fallback_model(model)` — `--fallback-model` for overload resilience
- [ ] `.no_session_persistence()` — `--no-session-persistence`

### Session Management

- [ ] `ClaudeCode::resume(session_id)` — `--resume <id>` with `-p` for non-interactive resume
- [ ] `ClaudeCode::continue_last()` — `--continue` with `-p`
- [ ] `.fork_session()` — `--fork-session` modifier on resume/continue

### Output Types (all Serialize + Deserialize)

- [ ] `PromptResult` — parsed from `--output-format json`:
  - `response: String` — the assistant's text response
  - `session_id: String` — for continuation
  - `cost_usd: Option<f64>` — cost if available
  - `duration_ms: Option<u64>` — execution time
  - `is_error: bool` — whether the result is an error
  - `model: String` — model used
  - `num_turns: u32` — conversation turns
- [ ] `StreamEvent` — parsed from `--output-format stream-json`:
  - Enum variants for different event types (message_start, content_block_delta, tool_use, result, etc.)
- [ ] `PromptTextResult` — simple wrapper for `--output-format text` (just the response string)

### Error Types

- [ ] `ClaudeError` enum:
  - `NotAuthenticated` — auth issues
  - `ModelNotAvailable { model: String }` — invalid/unavailable model
  - `BudgetExceeded { budget_usd: f64 }` — hit spending limit
  - `SessionNotFound { session_id: String }` — resume target doesn't exist
  - `SchemaValidationFailed { reason: String }` — JSON schema output didn't validate
  - `PermissionDenied { tool: String }` — tool use blocked by permission mode
  - `Command(CommandError)` — generic CLI failure
  - `Io(std::io::Error)` — process spawn failures
  - `Parse(serde_json::Error)` — JSON parsing failures

### Auth

- [ ] `ClaudeCode::auth_status()` — `claude auth status`, returns `AuthStatus`
- [ ] `AuthStatus` type: `logged_in: bool`, `account: Option<String>`

### Serialization

- [ ] All input builder configs serializable to a struct (for workflow checkpointing)
- [ ] `PromptConfig` — serializable snapshot of a `PromptBuilder`'s state
- [ ] `impl From<PromptConfig> for PromptBuilder` — reconstruct builder from config

### Integration

- [ ] `CliTool` impl for `ClaudeCode` (program = "claude")
- [ ] Workspace dependency in root `Cargo.toml`
- [ ] Feature flag in `fluent-sdlc` (`claude = ["dep:fluent-claude"]`)
- [ ] Sync execution (`blocking` feature)
- [ ] Async execution (`tokio` feature)

### File Structure

```
crates/fluent-claude/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── ops/
    │   ├── mod.rs
    │   ├── prompt.rs         # PromptBuilder, config, command building
    │   ├── session.rs        # SessionResumeBuilder, continue
    │   └── auth.rs           # AuthStatusBuilder
    ├── error/
    │   ├── mod.rs            # ClaudeError umbrella
    │   ├── prompt.rs         # PromptError
    │   └── auth.rs           # AuthError
    ├── types/
    │   ├── mod.rs
    │   ├── claude_code.rs    # ClaudeCode entry point
    │   ├── prompt.rs         # PromptResult, StreamEvent, PromptConfig
    │   ├── session.rs        # Session-related types
    │   └── auth.rs           # AuthStatus
    ├── sync/claude/ops/      # Feature-gated sync impls
    │   └── *.rs
    └── claude/ops/           # Feature-gated async impls
        └── *.rs
```

## Design Notes

### Composability

Builder inputs are captured in a serializable `PromptConfig` struct, enabling:
- Workflow engines to checkpoint and replay invocations
- Configuration to be stored in databases or message queues
- Dry-run mode (build config without executing)

### Output Format Strategy

- Default to `--output-format json` for programmatic use (single structured result)
- `stream-json` support returns an iterator/stream of `StreamEvent` for real-time processing
- `text` mode available as escape hatch for simple string responses

### Session Continuation

Session IDs from `PromptResult` can be fed back into `ClaudeCode::resume()` to build multi-turn workflows where each turn is a separate, checkpointable invocation.

### Error Classification

Errors are classified from a combination of:
- Exit code (non-zero = failure)
- JSON error fields in `--output-format json` responses (`is_error: true`)
- Stderr pattern matching for auth/permission issues
