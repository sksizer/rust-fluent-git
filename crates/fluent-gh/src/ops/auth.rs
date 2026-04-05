//! Builders for `gh auth` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::AuthError;
use crate::types::{AuthStatus, GitHub};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Entry Point ──────────────────────────────────────────────────────

/// Entry point builder for auth operations.
pub struct AuthBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> AuthBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Check authentication status.
    pub fn status(self) -> AuthStatusBuilder<'a> {
        AuthStatusBuilder::new(self.gh)
    }
}

// ── Status ───────────────────────────────────────────────────────────

/// Builder for `gh auth status`.
pub struct AuthStatusBuilder<'a> {
    #[allow(dead_code)]
    gh: &'a GitHub,
    hostname: Option<String>,
}

impl<'a> AuthStatusBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, hostname: None }
    }

    /// Set the hostname to check (default: github.com).
    pub fn hostname(mut self, hostname: impl Into<String>) -> Self {
        self.hostname = Some(hostname.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("auth").arg("status");

        let host = self.hostname.as_deref().unwrap_or("github.com");
        cmd = cmd.arg("--hostname").arg(host);

        cmd
    }
}

pub(crate) fn parse_status_output(output: &Output) -> Result<AuthStatus, AuthError> {
    // gh auth status outputs to stderr, and exits 0 if logged in, 1 if not
    let stderr = stderr_string(output);
    let stdout = stdout_string(output);
    let combined = format!("{stdout}{stderr}");
    let lower = combined.to_lowercase();

    if lower.contains("not logged") || lower.contains("no oauth") || !output.status.success() {
        if lower.contains("not logged") || lower.contains("no oauth") {
            return Err(AuthError::NotAuthenticated);
        }
        return Err(AuthError::Command(CommandError::Failed {
            args: "auth status".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout,
            stderr,
        }));
    }

    // Parse "Logged in to <host> as <user>"
    let host = extract_field(&combined, "Logged in to ", " ").unwrap_or_else(|| "github.com".to_string());
    let user =
        extract_field(&combined, " as ", " ").or_else(|| extract_field(&combined, " as ", "\n")).unwrap_or_default();
    let token_valid = lower.contains("token valid") || lower.contains("logged in");

    Ok(AuthStatus { user, host, token_valid })
}

fn extract_field(text: &str, prefix: &str, suffix: &str) -> Option<String> {
    let start = text.find(prefix)? + prefix.len();
    let remaining = &text[start..];
    let end = remaining.find(suffix).unwrap_or(remaining.len());
    let value = remaining[..end].trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}
