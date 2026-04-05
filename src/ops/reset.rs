//! Builder for `git reset` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, ResetError};
use crate::run::{stderr_string, stdout_string};
use crate::types::{ResetMode, ResetResult};

/// Builder for a `git reset` command.
pub struct ResetBuilder<'a> {
    repo_path: &'a Path,
    mode: ResetMode,
    target: Option<String>,
}

impl<'a> ResetBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path, mode: ResetMode::Mixed, target: None }
    }

    /// Use soft reset (keep changes staged).
    pub fn soft(mut self) -> Self {
        self.mode = ResetMode::Soft;
        self
    }

    /// Use mixed reset (unstage changes, keep working tree).
    pub fn mixed(mut self) -> Self {
        self.mode = ResetMode::Mixed;
        self
    }

    /// Use hard reset (discard all changes).
    pub fn hard(mut self) -> Self {
        self.mode = ResetMode::Hard;
        self
    }

    /// Set the target ref to reset to.
    pub fn to(mut self, reference: impl Into<String>) -> Self {
        self.target = Some(reference.into());
        self
    }

    pub(crate) fn build_reset_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("reset");

        match self.mode {
            ResetMode::Soft => cmd = cmd.arg("--soft"),
            ResetMode::Mixed => cmd = cmd.arg("--mixed"),
            ResetMode::Hard => cmd = cmd.arg("--hard"),
        }

        if let Some(ref target) = self.target {
            cmd = cmd.arg(target.as_str());
        }

        cmd
    }

    pub(crate) fn build_rev_parse_command(&self) -> ShellCommand {
        ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("rev-parse").arg("HEAD")
    }

    pub(crate) fn mode(&self) -> &ResetMode {
        &self.mode
    }

    pub(crate) fn target_ref(&self) -> Option<String> {
        self.target.clone()
    }
}

/// Parse the reset command output for errors.
pub(crate) fn parse_reset_output(output: &Output, target: &Option<String>) -> Result<(), ResetError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    if stderr.contains("unknown revision") || stderr.contains("ambiguous argument") {
        let reference = target.clone().unwrap_or_default();
        return Err(ResetError::RefNotFound { reference });
    }

    if stderr.contains("uncommitted merge") {
        return Err(ResetError::UncommittedMerge);
    }

    Err(ResetError::Command(CommandError::Failed {
        args: "reset".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

/// Parse the rev-parse output to get the new HEAD sha.
pub(crate) fn parse_rev_parse_for_reset(output: &Output, mode: &ResetMode) -> Result<ResetResult, ResetError> {
    let sha = stdout_string(output);
    Ok(ResetResult { sha, mode: mode.clone() })
}
