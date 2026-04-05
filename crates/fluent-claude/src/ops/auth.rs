//! Builder for `claude auth status` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::AuthError;
use crate::types::AuthStatus;
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── AuthStatusBuilder ────────────────────────────────────────────────

/// Builder for checking Claude Code authentication status.
///
/// No lifetime parameter — all data is owned.
pub struct AuthStatusBuilder {
    _private: (),
}

impl AuthStatusBuilder {
    /// Create a new auth status builder.
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("claude").arg("auth").arg("status")
    }
}

impl Default for AuthStatusBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn parse_status_output(output: &Output) -> Result<AuthStatus, AuthError> {
    let stdout = stdout_string(output);
    let stderr = stderr_string(output);

    if output.status.success() {
        let combined = format!("{stdout} {stderr}").to_lowercase();

        if combined.contains("logged in") {
            // Try to extract account info from the output.
            let account = extract_account(&stdout).or_else(|| extract_account(&stderr));
            return Ok(AuthStatus { logged_in: true, account });
        }

        // Success exit code but no "logged in" — treat as not authenticated.
        return Ok(AuthStatus { logged_in: false, account: None });
    }

    let lower = stderr.to_lowercase();

    if lower.contains("not logged in") || lower.contains("not authenticated") || lower.contains("no auth") {
        return Ok(AuthStatus { logged_in: false, account: None });
    }

    Err(AuthError::Command(CommandError::Failed {
        args: "claude auth status".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout,
        stderr,
    }))
}

/// Best-effort extraction of account name from CLI output.
fn extract_account(text: &str) -> Option<String> {
    // Look for patterns like "account: foo@bar.com" or "Logged in as foo@bar.com"
    let lower = text.to_lowercase();
    if let Some(pos) = lower.find("as ") {
        let rest = &text[pos + 3..];
        let account = rest.split_whitespace().next()?;
        if !account.is_empty() {
            return Some(account.trim_end_matches('.').to_string());
        }
    }
    if let Some(pos) = lower.find("account:") {
        let rest = &text[pos + 8..];
        let account = rest.split_whitespace().next()?;
        if !account.is_empty() {
            return Some(account.to_string());
        }
    }
    None
}
