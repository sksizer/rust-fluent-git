use crate::ops::formula::{
    InstallBuilder, LinkBuilder, PinBuilder, ReinstallBuilder, UninstallBuilder, UnlinkBuilder, UnpinBuilder,
    UpgradeBuilder,
};
use crate::ops::maintenance::{AutoremoveBuilder, CleanupBuilder, DoctorBuilder, UpdateBuilder};
use crate::ops::query::{DepsBuilder, InfoBuilder, ListBuilder, OutdatedBuilder, SearchBuilder};
use crate::ops::services::{
    ServicesInfoBuilder, ServicesKillBuilder, ServicesListBuilder, ServicesRestartBuilder, ServicesRunBuilder,
    ServicesStartBuilder, ServicesStopBuilder,
};
use crate::ops::tap::{TapBuilder, UntapBuilder};

/// Entry point for Homebrew CLI operations.
///
/// Stateless — configuration is per-invocation.
#[derive(Debug, Clone, Default)]
pub struct Brew;

impl Brew {
    pub fn new() -> Self {
        Self
    }

    // ── Formula operations ──────────────────────────────────────────

    /// Install a formula or cask.
    pub fn install(&self, name: impl Into<String>) -> InstallBuilder {
        InstallBuilder::new(name)
    }

    /// Uninstall a formula or cask.
    pub fn uninstall(&self, name: impl Into<String>) -> UninstallBuilder {
        UninstallBuilder::new(name)
    }

    /// Reinstall a formula or cask.
    pub fn reinstall(&self, name: impl Into<String>) -> ReinstallBuilder {
        ReinstallBuilder::new(name)
    }

    /// Upgrade a specific formula or cask, or all if no name given.
    pub fn upgrade(&self) -> UpgradeBuilder {
        UpgradeBuilder::new()
    }

    /// Pin a formula to prevent it from being upgraded.
    pub fn pin(&self, name: impl Into<String>) -> PinBuilder {
        PinBuilder::new(name)
    }

    /// Unpin a formula to allow it to be upgraded.
    pub fn unpin(&self, name: impl Into<String>) -> UnpinBuilder {
        UnpinBuilder::new(name)
    }

    /// Create symlinks for a keg-only formula.
    pub fn link(&self, name: impl Into<String>) -> LinkBuilder {
        LinkBuilder::new(name)
    }

    /// Remove symlinks for a formula.
    pub fn unlink(&self, name: impl Into<String>) -> UnlinkBuilder {
        UnlinkBuilder::new(name)
    }

    // ── Query operations ────────────────────────────────────────────

    /// Look up formula/cask info.
    pub fn info(&self, name: impl Into<String>) -> InfoBuilder {
        InfoBuilder::new(name)
    }

    /// Search for formulae/casks.
    pub fn search(&self, text: impl Into<String>) -> SearchBuilder {
        SearchBuilder::new(text)
    }

    /// List installed formulae/casks.
    pub fn list(&self) -> ListBuilder {
        ListBuilder::new()
    }

    /// List outdated formulae/casks.
    pub fn outdated(&self) -> OutdatedBuilder {
        OutdatedBuilder::new()
    }

    /// List dependencies of a formula.
    pub fn deps(&self, name: impl Into<String>) -> DepsBuilder {
        DepsBuilder::new(name)
    }

    // ── Tap operations ──────────────────────────────────────────────

    /// Tap a formula repository.
    pub fn tap(&self, repo: impl Into<String>) -> TapBuilder {
        TapBuilder::new(repo)
    }

    /// Untap a formula repository.
    pub fn untap(&self, repo: impl Into<String>) -> UntapBuilder {
        UntapBuilder::new(repo)
    }

    // ── Service operations ──────────────────────────────────────────

    /// List all managed services.
    pub fn services_list(&self) -> ServicesListBuilder {
        ServicesListBuilder::new()
    }

    /// Get info about a managed service.
    pub fn services_info(&self, name: impl Into<String>) -> ServicesInfoBuilder {
        ServicesInfoBuilder::new(name)
    }

    /// Start a service.
    pub fn services_start(&self, name: impl Into<String>) -> ServicesStartBuilder {
        ServicesStartBuilder::new(name)
    }

    /// Stop a service.
    pub fn services_stop(&self, name: impl Into<String>) -> ServicesStopBuilder {
        ServicesStopBuilder::new(name)
    }

    /// Restart a service.
    pub fn services_restart(&self, name: impl Into<String>) -> ServicesRestartBuilder {
        ServicesRestartBuilder::new(name)
    }

    /// Run a service (without registering for launch at login).
    pub fn services_run(&self, name: impl Into<String>) -> ServicesRunBuilder {
        ServicesRunBuilder::new(name)
    }

    /// Kill a service immediately.
    pub fn services_kill(&self, name: impl Into<String>) -> ServicesKillBuilder {
        ServicesKillBuilder::new(name)
    }

    // ── Maintenance operations ──────────────────────────────────────

    /// Update Homebrew and all formulae.
    pub fn update(&self) -> UpdateBuilder {
        UpdateBuilder::new()
    }

    /// Remove stale lock files and outdated downloads.
    pub fn cleanup(&self) -> CleanupBuilder {
        CleanupBuilder::new()
    }

    /// Uninstall formulae that are no longer needed.
    pub fn autoremove(&self) -> AutoremoveBuilder {
        AutoremoveBuilder::new()
    }

    /// Check system for potential problems.
    pub fn doctor(&self) -> DoctorBuilder {
        DoctorBuilder::new()
    }
}

impl fluent_core::tool::CliTool for Brew {
    fn program() -> &'static str {
        "brew"
    }
}
