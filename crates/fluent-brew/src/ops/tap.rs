//! Builders for `brew tap` and `brew untap` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::TapError;
use fluent_core::{CommandError, stderr_string, stdout_string};

fn classify_stderr(stderr: &str, name: &str, args: &str, output: &Output) -> TapError {
    let lower = stderr.to_lowercase();
    if lower.contains("already tapped") {
        return TapError::AlreadyTapped { name: name.to_string() };
    }
    if lower.contains("no available tap") || lower.contains("not tapped") {
        return TapError::NotTapped { name: name.to_string() };
    }
    TapError::Command(CommandError::Failed {
        args: args.to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr: stderr.to_string(),
    })
}

// ── Tap ───────────────────────────────────────────────────────────────

/// Builder for `brew tap user/repo [url]`.
pub struct TapBuilder {
    repo: String,
    url: Option<String>,
    force_auto_update: bool,
}

impl TapBuilder {
    pub(crate) fn new(repo: impl Into<String>) -> Self {
        Self { repo: repo.into(), url: None, force_auto_update: false }
    }

    /// Set a custom URL for the tap.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Pass `--force-auto-update` to `brew tap`.
    pub fn force_auto_update(mut self) -> Self {
        self.force_auto_update = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("tap");

        if self.force_auto_update {
            cmd = cmd.arg("--force-auto-update");
        }

        cmd = cmd.arg(&self.repo);

        if let Some(ref url) = self.url {
            cmd = cmd.arg(url.as_str());
        }

        cmd
    }

    pub(crate) fn tap_name(&self) -> &str {
        &self.repo
    }
}

pub(crate) fn parse_tap_output(output: &Output, name: &str) -> Result<(), TapError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew tap {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}

// ── Untap ─────────────────────────────────────────────────────────────

/// Builder for `brew untap user/repo`.
pub struct UntapBuilder {
    repo: String,
    force: bool,
}

impl UntapBuilder {
    pub(crate) fn new(repo: impl Into<String>) -> Self {
        Self { repo: repo.into(), force: false }
    }

    /// Pass `--force` to `brew untap`.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("untap");

        if self.force {
            cmd = cmd.arg("--force");
        }

        cmd = cmd.arg(&self.repo);

        cmd
    }

    pub(crate) fn tap_name(&self) -> &str {
        &self.repo
    }
}

pub(crate) fn parse_untap_output(output: &Output, name: &str) -> Result<(), TapError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew untap {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}
