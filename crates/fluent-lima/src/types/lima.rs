use crate::ops::copy::CopyBuilder;
use crate::ops::disk::DiskBuilder;
use crate::ops::info::InfoBuilder;
use crate::ops::instance::{
    CloneBuilder, CreateBuilder, DeleteBuilder, ListBuilder, ProtectBuilder, RenameBuilder, RestartBuilder,
    StartBuilder, StopBuilder, UnprotectBuilder,
};
use crate::ops::shell::ShellBuilder;
use crate::ops::snapshot::SnapshotBuilder;

/// Entry point for Lima (limactl) CLI operations.
///
/// Stateless — configuration is per-invocation.
#[derive(Debug, Clone, Default)]
pub struct Lima;

impl Lima {
    pub fn new() -> Self {
        Self
    }

    /// Start building a VM creation invocation.
    pub fn create(&self) -> CreateBuilder {
        CreateBuilder::new()
    }

    /// Start a VM instance.
    pub fn start(&self, instance: impl Into<String>) -> StartBuilder {
        StartBuilder::new(instance)
    }

    /// Stop a VM instance.
    pub fn stop(&self, instance: impl Into<String>) -> StopBuilder {
        StopBuilder::new(instance)
    }

    /// Restart a VM instance.
    pub fn restart(&self, instance: impl Into<String>) -> RestartBuilder {
        RestartBuilder::new(instance)
    }

    /// Delete a VM instance.
    pub fn delete(&self, instance: impl Into<String>) -> DeleteBuilder {
        DeleteBuilder::new(instance)
    }

    /// List VM instances.
    pub fn list(&self) -> ListBuilder {
        ListBuilder::new()
    }

    /// Clone a VM instance.
    pub fn clone_instance(&self, source: impl Into<String>, dest: impl Into<String>) -> CloneBuilder {
        CloneBuilder::new(source, dest)
    }

    /// Rename a VM instance.
    pub fn rename(&self, old: impl Into<String>, new: impl Into<String>) -> RenameBuilder {
        RenameBuilder::new(old, new)
    }

    /// Protect a VM instance from accidental deletion.
    pub fn protect(&self, instance: impl Into<String>) -> ProtectBuilder {
        ProtectBuilder::new(instance)
    }

    /// Remove protection from a VM instance.
    pub fn unprotect(&self, instance: impl Into<String>) -> UnprotectBuilder {
        UnprotectBuilder::new(instance)
    }

    /// Execute a command in a VM instance.
    pub fn shell(&self, instance: impl Into<String>) -> ShellBuilder {
        ShellBuilder::new(instance)
    }

    /// Copy files between host and guest.
    pub fn copy(&self, source: impl Into<String>, target: impl Into<String>) -> CopyBuilder {
        CopyBuilder::new(source, target)
    }

    /// Manage snapshots for a VM instance.
    pub fn snapshot(&self, instance: impl Into<String>) -> SnapshotBuilder {
        SnapshotBuilder::new(instance)
    }

    /// Access disk management sub-commands.
    pub fn disk(&self) -> DiskBuilder {
        DiskBuilder::new()
    }

    /// Query system-level Lima information.
    pub fn info(&self) -> InfoBuilder {
        InfoBuilder::new()
    }
}

impl fluent_core::tool::CliTool for Lima {
    fn program() -> &'static str {
        "limactl"
    }
}
