//! Shared error types for CLI command execution.

/// Fallback error for when a CLI command exits non-zero in a way
/// that wasn't categorized into a domain-specific error variant.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("command `{args}` failed (exit code {code}):\n{stderr}")]
    Failed { args: String, code: i32, stdout: String, stderr: String },

    #[error("command `{args}` timed out after {timeout_secs}s")]
    Timeout { args: String, timeout_secs: u64 },

    #[error("command `{args}` was interrupted by signal {signal}")]
    Signal { args: String, signal: i32 },
}
