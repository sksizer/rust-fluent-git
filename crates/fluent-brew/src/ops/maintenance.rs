//! Builders for `brew update`, `brew cleanup`, `brew autoremove`, and `brew doctor` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::MaintenanceError;
use fluent_core::{CommandError, stderr_string, stdout_string};

// ── Update ───────────────────────────────────────────────────────────

/// Builder for `brew update`.
pub struct UpdateBuilder {
    force: bool,
    quiet: bool,
}

impl UpdateBuilder {
    pub(crate) fn new() -> Self {
        Self { force: false, quiet: false }
    }

    /// Pass `--force` to `brew update`.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Pass `--quiet` to `brew update`.
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("update");

        if self.force {
            cmd = cmd.arg("--force");
        }

        if self.quiet {
            cmd = cmd.arg("--quiet");
        }

        cmd
    }
}

pub(crate) fn parse_update_output(output: &Output) -> Result<(), MaintenanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    Err(MaintenanceError::Command(CommandError::Failed {
        args: "brew update".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Cleanup ──────────────────────────────────────────────────────────

/// Builder for `brew cleanup [options] [formula|cask]`.
pub struct CleanupBuilder {
    dry_run: bool,
    prune_days: Option<u32>,
    name: Option<String>,
}

impl CleanupBuilder {
    pub(crate) fn new() -> Self {
        Self { dry_run: false, prune_days: None, name: None }
    }

    /// Pass `--dry-run` to `brew cleanup`.
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// Pass `--prune=N` to `brew cleanup`.
    pub fn prune_days(mut self, days: u32) -> Self {
        self.prune_days = Some(days);
        self
    }

    /// Specify a particular formula or cask to clean up.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("cleanup");

        if self.dry_run {
            cmd = cmd.arg("--dry-run");
        }

        if let Some(days) = self.prune_days {
            cmd = cmd.arg(format!("--prune={days}"));
        }

        if let Some(ref name) = self.name {
            cmd = cmd.arg(name.as_str());
        }

        cmd
    }
}

pub(crate) fn parse_cleanup_output(output: &Output) -> Result<(), MaintenanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    Err(MaintenanceError::Command(CommandError::Failed {
        args: "brew cleanup".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Autoremove ───────────────────────────────────────────────────────

/// Builder for `brew autoremove`.
pub struct AutoremoveBuilder {
    dry_run: bool,
}

impl AutoremoveBuilder {
    pub(crate) fn new() -> Self {
        Self { dry_run: false }
    }

    /// Pass `--dry-run` to `brew autoremove`.
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("autoremove");

        if self.dry_run {
            cmd = cmd.arg("--dry-run");
        }

        cmd
    }
}

pub(crate) fn parse_autoremove_output(output: &Output) -> Result<(), MaintenanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    Err(MaintenanceError::Command(CommandError::Failed {
        args: "brew autoremove".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Doctor ───────────────────────────────────────────────────────────

/// Builder for `brew doctor`.
pub struct DoctorBuilder {
    _private: (),
}

impl DoctorBuilder {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("doctor")
    }
}

/// Parse doctor output. Doctor exits 0 when no issues are found, and non-zero
/// when issues exist. Both are valid results — we always return the stdout text.
/// Only actual IO/command-execution failures produce an error.
pub(crate) fn parse_doctor_output(output: &Output) -> Result<String, MaintenanceError> {
    Ok(stdout_string(output))
}
