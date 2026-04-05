//! Builders for `brew services` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::ServiceError;
use crate::types::ServiceInfo;
use fluent_core::{CommandError, stderr_string, stdout_string};

fn classify_stderr(stderr: &str, name: &str, args: &str, output: &Output) -> ServiceError {
    let lower = stderr.to_lowercase();
    if lower.contains("not installed") || lower.contains("unknown service") {
        return ServiceError::NotFound { name: name.to_string() };
    }
    if lower.contains("already started") || lower.contains("already running") {
        return ServiceError::AlreadyRunning { name: name.to_string() };
    }
    if lower.contains("is not started") || lower.contains("not running") {
        return ServiceError::NotRunning { name: name.to_string() };
    }
    ServiceError::Command(CommandError::Failed {
        args: args.to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr: stderr.to_string(),
    })
}

// ── List ────────────────────────────────────────────────────────────────

/// Builder for `brew services list --json`.
pub struct ServicesListBuilder;

impl ServicesListBuilder {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("services").arg("list").arg("--json")
    }
}

pub(crate) fn parse_services_list_output(output: &Output) -> Result<Vec<ServiceInfo>, ServiceError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        let args = "brew services list --json".to_string();
        return Err(ServiceError::Command(CommandError::Failed {
            args,
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let services: Vec<ServiceInfo> = serde_json::from_str(&stdout)?;
    Ok(services)
}

// ── Info ────────────────────────────────────────────────────────────────

/// Builder for `brew services info formula --json`.
pub struct ServicesInfoBuilder {
    name: String,
}

impl ServicesInfoBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("services").arg("info").arg("--json").arg(&self.name)
    }

    pub(crate) fn service_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_services_info_output(output: &Output, name: &str) -> Result<Vec<ServiceInfo>, ServiceError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        let args = format!("brew services info --json {name}");
        return Err(classify_stderr(&stderr, name, &args, output));
    }

    let stdout = stdout_string(output);
    let services: Vec<ServiceInfo> = serde_json::from_str(&stdout)?;
    Ok(services)
}

// ── Start ───────────────────────────────────────────────────────────────

/// Builder for `brew services start formula`.
pub struct ServicesStartBuilder {
    name: String,
    file: Option<String>,
}

impl ServicesStartBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), file: None }
    }

    /// Set the `--file=` option.
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("services").arg("start");

        if let Some(ref file) = self.file {
            cmd = cmd.arg(format!("--file={file}"));
        }

        cmd = cmd.arg(&self.name);
        cmd
    }

    pub(crate) fn service_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_services_start_output(output: &Output, name: &str) -> Result<(), ServiceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew services start {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}

// ── Stop ────────────────────────────────────────────────────────────────

/// Builder for `brew services stop formula`.
pub struct ServicesStopBuilder {
    name: String,
    keep: bool,
}

impl ServicesStopBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), keep: false }
    }

    /// Pass the `--keep` flag to `brew services stop`.
    pub fn keep(mut self) -> Self {
        self.keep = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("services").arg("stop");

        if self.keep {
            cmd = cmd.arg("--keep");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }

    pub(crate) fn service_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_services_stop_output(output: &Output, name: &str) -> Result<(), ServiceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew services stop {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}

// ── Restart ─────────────────────────────────────────────────────────────

/// Builder for `brew services restart formula`.
pub struct ServicesRestartBuilder {
    name: String,
    file: Option<String>,
}

impl ServicesRestartBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), file: None }
    }

    /// Set the `--file=` option.
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("services").arg("restart");

        if let Some(ref file) = self.file {
            cmd = cmd.arg(format!("--file={file}"));
        }

        cmd = cmd.arg(&self.name);
        cmd
    }

    pub(crate) fn service_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_services_restart_output(output: &Output, name: &str) -> Result<(), ServiceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew services restart {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}

// ── Run ─────────────────────────────────────────────────────────────────

/// Builder for `brew services run formula`.
pub struct ServicesRunBuilder {
    name: String,
    file: Option<String>,
}

impl ServicesRunBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), file: None }
    }

    /// Set the `--file=` option.
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("services").arg("run");

        if let Some(ref file) = self.file {
            cmd = cmd.arg(format!("--file={file}"));
        }

        cmd = cmd.arg(&self.name);
        cmd
    }

    pub(crate) fn service_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_services_run_output(output: &Output, name: &str) -> Result<(), ServiceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew services run {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}

// ── Kill ────────────────────────────────────────────────────────────────

/// Builder for `brew services kill formula`.
pub struct ServicesKillBuilder {
    name: String,
}

impl ServicesKillBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("services").arg("kill").arg(&self.name)
    }

    pub(crate) fn service_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_services_kill_output(output: &Output, name: &str) -> Result<(), ServiceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let args = format!("brew services kill {name}");
    Err(classify_stderr(&stderr, name, &args, output))
}
