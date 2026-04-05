//! Builders for `limactl disk` sub-commands.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::DiskError;
use crate::types::DiskInfo;
use fluent_core::{CommandError, stderr_string, stdout_string};

// ── Helper ───────────────────────────────────────────────────────────

/// Classify stderr into a typed disk error, falling back to a generic command error.
fn classify_stderr(stderr: &str, name: &str, args: &str, output: &Output) -> DiskError {
    let lower = stderr.to_lowercase();

    if lower.contains("does not exist") || lower.contains("not found") {
        return DiskError::NotFound { name: name.to_string() };
    }

    if lower.contains("already exists") {
        return DiskError::AlreadyExists { name: name.to_string() };
    }

    if lower.contains("in use") || lower.contains("currently used") {
        return DiskError::InUse { name: name.to_string() };
    }

    DiskError::Command(CommandError::Failed {
        args: args.to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr: stderr.to_string(),
    })
}

// ── DiskBuilder (entry) ─────────────────────────────────────────────

/// Entry point for `limactl disk` sub-commands.
pub struct DiskBuilder;

impl DiskBuilder {
    pub(crate) fn new() -> Self {
        Self
    }

    /// Create a new disk.
    pub fn create(self, name: impl Into<String>) -> DiskCreateBuilder {
        DiskCreateBuilder::new(name)
    }

    /// List all disks.
    pub fn list(self) -> DiskListBuilder {
        DiskListBuilder::new()
    }

    /// Delete a disk.
    pub fn delete(self, name: impl Into<String>) -> DiskDeleteBuilder {
        DiskDeleteBuilder::new(name)
    }

    /// Resize a disk.
    pub fn resize(self, name: impl Into<String>) -> DiskResizeBuilder {
        DiskResizeBuilder::new(name)
    }
}

// ── Create ───────────────────────────────────────────────────────────

pub struct DiskCreateBuilder {
    name: String,
    size: String,
    format: Option<String>,
}

impl DiskCreateBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), size: "10GiB".to_string(), format: None }
    }

    /// Set the disk size (e.g. "10GiB", "50GiB").
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the disk format (e.g. "qcow2", "raw").
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub(crate) fn disk_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("limactl").arg("disk").arg("create").arg("--tty=false").arg("--size").arg(&self.size);

        if let Some(ref fmt) = self.format {
            cmd = cmd.arg("--format").arg(fmt);
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_create_output(output: &Output, name: &str) -> Result<(), DiskError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl disk create {name}"), output))
}

// ── List ─────────────────────────────────────────────────────────────

pub struct DiskListBuilder;

impl DiskListBuilder {
    fn new() -> Self {
        Self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("disk").arg("ls").arg("--json").arg("--tty=false")
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<DiskInfo>, DiskError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let mut disks = Vec::new();
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let info: DiskInfo = serde_json::from_str(trimmed)?;
            disks.push(info);
        }
        return Ok(disks);
    }

    let stderr = stderr_string(output);
    Err(DiskError::Command(CommandError::Failed {
        args: "limactl disk ls --json".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ───────────────────────────────────────────────────────────

pub struct DiskDeleteBuilder {
    name: String,
}

impl DiskDeleteBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn disk_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("disk").arg("delete").arg("--tty=false").arg(&self.name)
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str) -> Result<(), DiskError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl disk delete {name}"), output))
}

// ── Resize ───────────────────────────────────────────────────────────

pub struct DiskResizeBuilder {
    name: String,
    size: String,
}

impl DiskResizeBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), size: "10GiB".to_string() }
    }

    /// Set the new disk size (e.g. "50GiB").
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = size.into();
        self
    }

    pub(crate) fn disk_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl")
            .arg("disk")
            .arg("resize")
            .arg("--tty=false")
            .arg("--size")
            .arg(&self.size)
            .arg(&self.name)
    }
}

pub(crate) fn parse_resize_output(output: &Output, name: &str) -> Result<(), DiskError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl disk resize {name}"), output))
}
