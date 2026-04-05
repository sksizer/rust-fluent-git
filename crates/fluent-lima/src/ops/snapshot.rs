//! Builders for `limactl snapshot` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::SnapshotError;
use crate::types::SnapshotInfo;
use fluent_core::stderr_string;

// ── SnapshotBuilder (entry point) ───────────────────────────────────

pub struct SnapshotBuilder {
    instance: String,
}

impl SnapshotBuilder {
    pub(crate) fn new(instance: impl Into<String>) -> Self {
        Self { instance: instance.into() }
    }

    pub fn create(self, tag: impl Into<String>) -> SnapshotCreateBuilder {
        SnapshotCreateBuilder { instance: self.instance, tag: tag.into() }
    }

    pub fn apply(self, tag: impl Into<String>) -> SnapshotApplyBuilder {
        SnapshotApplyBuilder { instance: self.instance, tag: tag.into() }
    }

    pub fn delete(self, tag: impl Into<String>) -> SnapshotDeleteBuilder {
        SnapshotDeleteBuilder { instance: self.instance, tag: tag.into() }
    }

    pub fn list(self) -> SnapshotListBuilder {
        SnapshotListBuilder { instance: self.instance }
    }
}

// ── SnapshotCreateBuilder ───────────────────────────────────────────

pub struct SnapshotCreateBuilder {
    pub(crate) instance: String,
    pub(crate) tag: String,
}

impl SnapshotCreateBuilder {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl")
            .arg("snapshot")
            .arg("create")
            .arg("--tty=false")
            .arg("--tag")
            .arg(&self.tag)
            .arg(&self.instance)
    }
}

// ── SnapshotApplyBuilder ────────────────────────────────────────────

pub struct SnapshotApplyBuilder {
    pub(crate) instance: String,
    pub(crate) tag: String,
}

impl SnapshotApplyBuilder {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl")
            .arg("snapshot")
            .arg("apply")
            .arg("--tty=false")
            .arg("--tag")
            .arg(&self.tag)
            .arg(&self.instance)
    }
}

// ── SnapshotDeleteBuilder ───────────────────────────────────────────

pub struct SnapshotDeleteBuilder {
    pub(crate) instance: String,
    pub(crate) tag: String,
}

impl SnapshotDeleteBuilder {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl")
            .arg("snapshot")
            .arg("delete")
            .arg("--tty=false")
            .arg("--tag")
            .arg(&self.tag)
            .arg(&self.instance)
    }
}

// ── SnapshotListBuilder ─────────────────────────────────────────────

pub struct SnapshotListBuilder {
    pub(crate) instance: String,
}

impl SnapshotListBuilder {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("snapshot").arg("list").arg("--tty=false").arg(&self.instance)
    }
}

// ── Output Parsing ──────────────────────────────────────────────────

pub(crate) fn parse_snapshot_output(output: &Output, instance: &str, tag: Option<&str>) -> Result<(), SnapshotError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("does not exist") {
        return Err(SnapshotError::InstanceNotFound { name: instance.to_string() });
    }

    if lower.contains("not found") && lower.contains("tag") {
        return Err(SnapshotError::TagNotFound { tag: tag.unwrap_or_default().to_string() });
    }

    if lower.contains("already exists") {
        return Err(SnapshotError::TagAlreadyExists { tag: tag.unwrap_or_default().to_string() });
    }

    if lower.contains("must be stopped") || lower.contains("running") {
        return Err(SnapshotError::MustBeStopped { name: instance.to_string() });
    }

    Err(SnapshotError::Command(fluent_core::CommandError::Failed {
        args: "limactl snapshot".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: fluent_core::stdout_string(output),
        stderr,
    }))
}

pub(crate) fn parse_snapshot_list_output(output: &Output, instance: &str) -> Result<Vec<SnapshotInfo>, SnapshotError> {
    if output.status.success() {
        let stdout = fluent_core::stdout_string(output);
        let snapshots = stdout
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| SnapshotInfo { tag: line.to_string() })
            .collect();
        return Ok(snapshots);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("does not exist") {
        return Err(SnapshotError::InstanceNotFound { name: instance.to_string() });
    }

    if lower.contains("must be stopped") || lower.contains("running") {
        return Err(SnapshotError::MustBeStopped { name: instance.to_string() });
    }

    Err(SnapshotError::Command(fluent_core::CommandError::Failed {
        args: "limactl snapshot list".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: fluent_core::stdout_string(output),
        stderr,
    }))
}
