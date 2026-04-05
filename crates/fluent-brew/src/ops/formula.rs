//! Builders for `brew` formula lifecycle operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::FormulaError;
use fluent_core::{CommandError, stderr_string, stdout_string};

// ── Helper ───────────────────────────────────────────────────────────

/// Classify stderr into a typed error, falling back to a generic command error.
fn classify_stderr(stderr: &str, name: &str, args: &str, output: &Output) -> FormulaError {
    let lower = stderr.to_lowercase();

    if lower.contains("no available formula") || lower.contains("no available cask") {
        return FormulaError::NotFound { name: name.to_string() };
    }

    if lower.contains("already installed") {
        return FormulaError::AlreadyInstalled { name: name.to_string() };
    }

    if lower.contains("not installed") || lower.contains("no such keg") {
        return FormulaError::NotInstalled { name: name.to_string() };
    }

    if lower.contains("is not pinned") {
        return FormulaError::NotPinned { name: name.to_string() };
    }

    if lower.contains("already linked") {
        return FormulaError::AlreadyLinked { name: name.to_string() };
    }

    FormulaError::Command(CommandError::Failed {
        args: args.to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr: stderr.to_string(),
    })
}

// ── Install ─────────────────────────────────────────────────────────

pub struct InstallBuilder {
    name: String,
    cask: bool,
    force: bool,
    quiet: bool,
    no_quarantine: bool,
    head: bool,
    fetch_head: bool,
    build_from_source: bool,
}

impl InstallBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cask: false,
            force: false,
            quiet: false,
            no_quarantine: false,
            head: false,
            fetch_head: false,
            build_from_source: false,
        }
    }

    /// Install as a cask instead of a formula.
    pub fn cask(mut self) -> Self {
        self.cask = true;
        self
    }

    /// Force installation even if already installed.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Suppress output.
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Disable quarantine for casks.
    pub fn no_quarantine(mut self) -> Self {
        self.no_quarantine = true;
        self
    }

    /// Install the HEAD version.
    pub fn head(mut self) -> Self {
        self.head = true;
        self
    }

    /// Fetch the upstream repository to detect if the HEAD installation is outdated.
    pub fn fetch_head(mut self) -> Self {
        self.fetch_head = true;
        self
    }

    /// Compile from source even if a bottle is available.
    pub fn build_from_source(mut self) -> Self {
        self.build_from_source = true;
        self
    }

    /// Access the formula/cask name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("install").env("HOMEBREW_NO_AUTO_UPDATE", "1");

        if self.cask {
            cmd = cmd.arg("--cask");
        }
        if self.force {
            cmd = cmd.arg("--force");
        }
        if self.quiet {
            cmd = cmd.arg("--quiet");
        }
        if self.no_quarantine {
            cmd = cmd.arg("--no-quarantine");
        }
        if self.head {
            cmd = cmd.arg("--HEAD");
        }
        if self.fetch_head {
            cmd = cmd.arg("--fetch-HEAD");
        }
        if self.build_from_source {
            cmd = cmd.arg("--build-from-source");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_install_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew install {name}"), output))
}

// ── Uninstall ───────────────────────────────────────────────────────

pub struct UninstallBuilder {
    name: String,
    cask: bool,
    force: bool,
    zap: bool,
}

impl UninstallBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), cask: false, force: false, zap: false }
    }

    /// Uninstall as a cask.
    pub fn cask(mut self) -> Self {
        self.cask = true;
        self
    }

    /// Force uninstallation.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Remove all files associated with a cask (implies --cask).
    pub fn zap(mut self) -> Self {
        self.zap = true;
        self
    }

    /// Access the formula/cask name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("uninstall");

        if self.cask {
            cmd = cmd.arg("--cask");
        }
        if self.force {
            cmd = cmd.arg("--force");
        }
        if self.zap {
            cmd = cmd.arg("--zap");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_uninstall_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew uninstall {name}"), output))
}

// ── Reinstall ───────────────────────────────────────────────────────

pub struct ReinstallBuilder {
    name: String,
    cask: bool,
    force: bool,
    no_quarantine: bool,
}

impl ReinstallBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), cask: false, force: false, no_quarantine: false }
    }

    /// Reinstall as a cask.
    pub fn cask(mut self) -> Self {
        self.cask = true;
        self
    }

    /// Force reinstallation.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Disable quarantine for casks.
    pub fn no_quarantine(mut self) -> Self {
        self.no_quarantine = true;
        self
    }

    /// Access the formula/cask name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("reinstall");

        if self.cask {
            cmd = cmd.arg("--cask");
        }
        if self.force {
            cmd = cmd.arg("--force");
        }
        if self.no_quarantine {
            cmd = cmd.arg("--no-quarantine");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_reinstall_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew reinstall {name}"), output))
}

// ── Upgrade ─────────────────────────────────────────────────────────

pub struct UpgradeBuilder {
    name: Option<String>,
    cask: bool,
    force: bool,
    greedy: bool,
    dry_run: bool,
}

impl UpgradeBuilder {
    pub(crate) fn new() -> Self {
        Self { name: None, cask: false, force: false, greedy: false, dry_run: false }
    }

    /// Upgrade a specific formula or cask.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Upgrade casks.
    pub fn cask(mut self) -> Self {
        self.cask = true;
        self
    }

    /// Force upgrade.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Include casks with auto-update enabled (greedy mode).
    pub fn greedy(mut self) -> Self {
        self.greedy = true;
        self
    }

    /// Show what would be upgraded without actually upgrading.
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// Access the formula/cask name, if set.
    pub(crate) fn formula_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("upgrade");

        if self.cask {
            cmd = cmd.arg("--cask");
        }
        if self.force {
            cmd = cmd.arg("--force");
        }
        if self.greedy {
            cmd = cmd.arg("--greedy");
        }
        if self.dry_run {
            cmd = cmd.arg("--dry-run");
        }

        if let Some(ref name) = self.name {
            cmd = cmd.arg(name);
        }

        cmd
    }
}

pub(crate) fn parse_upgrade_output(output: &Output, name: Option<&str>) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let display_name = name.unwrap_or("all");
    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, display_name, &format!("brew upgrade {display_name}"), output))
}

// ── Pin ─────────────────────────────────────────────────────────────

pub struct PinBuilder {
    name: String,
}

impl PinBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Access the formula name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("pin").arg(&self.name)
    }
}

pub(crate) fn parse_pin_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew pin {name}"), output))
}

// ── Unpin ───────────────────────────────────────────────────────────

pub struct UnpinBuilder {
    name: String,
}

impl UnpinBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Access the formula name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("unpin").arg(&self.name)
    }
}

pub(crate) fn parse_unpin_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew unpin {name}"), output))
}

// ── Link ────────────────────────────────────────────────────────────

pub struct LinkBuilder {
    name: String,
    overwrite: bool,
    force: bool,
}

impl LinkBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), overwrite: false, force: false }
    }

    /// Overwrite existing files when linking.
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self
    }

    /// Force linking even if keg-only.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Access the formula name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("link");

        if self.overwrite {
            cmd = cmd.arg("--overwrite");
        }
        if self.force {
            cmd = cmd.arg("--force");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_link_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew link {name}"), output))
}

// ── Unlink ──────────────────────────────────────────────────────────

pub struct UnlinkBuilder {
    name: String,
}

impl UnlinkBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Access the formula name.
    pub(crate) fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("brew").arg("unlink").arg(&self.name)
    }
}

pub(crate) fn parse_unlink_output(output: &Output, name: &str) -> Result<(), FormulaError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("brew unlink {name}"), output))
}
