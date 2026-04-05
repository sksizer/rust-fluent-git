//! Builder for `limactl shell` operations.

#![allow(dead_code)]

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::ShellError;
use crate::types::ShellResult;
use fluent_core::{stderr_string, stdout_string};

// ── ShellBuilder ────────────────────────────────────────────────────

pub struct ShellBuilder {
    instance: String,
    command: Option<String>,
    args: Vec<String>,
    workdir: Option<String>,
    shell: Option<String>,
    preserve_env: bool,
    start: bool,
}

impl ShellBuilder {
    pub(crate) fn new(instance: impl Into<String>) -> Self {
        Self {
            instance: instance.into(),
            command: None,
            args: Vec::new(),
            workdir: None,
            shell: None,
            preserve_env: false,
            start: false,
        }
    }

    /// Set the command and its arguments at once.
    pub fn command(mut self, cmd: impl Into<String>, args: &[&str]) -> Self {
        self.command = Some(cmd.into());
        self.args = args.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Set the working directory inside the guest.
    pub fn workdir(mut self, dir: impl Into<String>) -> Self {
        self.workdir = Some(dir.into());
        self
    }

    /// Set the shell interpreter override.
    pub fn shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = Some(shell.into());
        self
    }

    /// Preserve the host environment variables in the guest.
    pub fn preserve_env(mut self) -> Self {
        self.preserve_env = true;
        self
    }

    /// Auto-start the instance if it is stopped.
    pub fn start(mut self) -> Self {
        self.start = true;
        self
    }

    /// Expose the instance name for sync/async execution layers.
    pub(crate) fn instance_name(&self) -> &str {
        &self.instance
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("limactl").arg("shell").arg("--tty=false");

        if let Some(ref dir) = self.workdir {
            cmd = cmd.arg("--workdir").arg(dir);
        }

        if let Some(ref sh) = self.shell {
            cmd = cmd.arg("--shell").arg(sh);
        }

        if self.preserve_env {
            cmd = cmd.arg("--preserve-env");
        }

        if self.start {
            cmd = cmd.arg("--start");
        }

        cmd = cmd.arg(&self.instance);

        if let Some(ref command) = self.command {
            cmd = cmd.arg("--").arg(command);
            for arg in &self.args {
                cmd = cmd.arg(arg);
            }
        }

        cmd
    }
}

// ── Output Parsing ──────────────────────────────────────────────────

pub(crate) fn parse_shell_output(output: &Output, name: &str) -> Result<ShellResult, ShellError> {
    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    // Check for lima-level errors first
    if lower.contains("does not exist") || lower.contains("not found") {
        return Err(ShellError::NotFound { name: name.to_string() });
    }

    if lower.contains("not running") {
        return Err(ShellError::NotRunning { name: name.to_string() });
    }

    let exit_code = output.status.code().unwrap_or(-1);

    // If we got here, the guest command ran (even if it returned non-zero)
    Ok(ShellResult { exit_code, stdout: stdout_string(output), stderr })
}
