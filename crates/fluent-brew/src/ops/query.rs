//! Builders for `brew` query operations: info, search, list, outdated, deps.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::QueryError;
use crate::types::{InfoResponse, OutdatedResponse};
use fluent_core::{CommandError, stdout_string};

// ── Info ────────────────────────────────────────────────────────────

/// Builder for `brew info --json=v2 <name>`.
pub struct InfoBuilder {
    name: String,
    cask: bool,
}

impl InfoBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), cask: false }
    }

    /// Restrict lookup to casks only.
    pub fn cask(mut self) -> Self {
        self.cask = true;
        self
    }

    /// The formula/cask name this builder targets.
    pub fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("info").arg("--json=v2").env("HOMEBREW_NO_AUTO_UPDATE", "1");

        if self.cask {
            cmd = cmd.arg("--cask");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_info_output(output: &Output, name: &str) -> Result<InfoResponse, QueryError> {
    if !output.status.success() {
        let stderr = fluent_core::stderr_string(output);
        if stderr.contains("No available formula")
            || stderr.contains("No available cask")
            || stderr.contains("not found")
        {
            return Err(QueryError::NotFound { name: name.to_string() });
        }
        return Err(QueryError::Command(CommandError::Failed {
            args: format!("brew info --json=v2 {name}"),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let resp: InfoResponse = serde_json::from_str(&stdout)?;
    Ok(resp)
}

// ── Search ──────────────────────────────────────────────────────────

/// Builder for `brew search <text>`.
pub struct SearchBuilder {
    text: String,
    cask: bool,
    formula: bool,
}

impl SearchBuilder {
    pub(crate) fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), cask: false, formula: false }
    }

    /// Restrict search to casks only.
    pub fn cask(mut self) -> Self {
        self.cask = true;
        self
    }

    /// Restrict search to formulae only.
    pub fn formula(mut self) -> Self {
        self.formula = true;
        self
    }

    /// The search text this builder targets.
    pub fn search_text(&self) -> &str {
        &self.text
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("search").env("HOMEBREW_NO_AUTO_UPDATE", "1");

        if self.formula {
            cmd = cmd.arg("--formulae");
        }
        if self.cask {
            cmd = cmd.arg("--cask");
        }

        cmd = cmd.arg(&self.text);
        cmd
    }
}

pub(crate) fn parse_search_output(output: &Output) -> Result<Vec<String>, QueryError> {
    if !output.status.success() {
        let stderr = fluent_core::stderr_string(output);
        return Err(QueryError::Command(CommandError::Failed {
            args: "brew search".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let results: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("==>"))
        .map(|line| line.trim().to_string())
        .collect();
    Ok(results)
}

// ── List ────────────────────────────────────────────────────────────

/// Builder for `brew list`.
pub struct ListBuilder {
    formulae_only: bool,
    casks_only: bool,
    versions: bool,
    pinned: bool,
}

impl ListBuilder {
    pub(crate) fn new() -> Self {
        Self { formulae_only: false, casks_only: false, versions: false, pinned: false }
    }

    /// Show only formulae.
    pub fn formulae_only(mut self) -> Self {
        self.formulae_only = true;
        self
    }

    /// Show only casks.
    pub fn casks_only(mut self) -> Self {
        self.casks_only = true;
        self
    }

    /// Include version info.
    pub fn versions(mut self) -> Self {
        self.versions = true;
        self
    }

    /// Show only pinned formulae.
    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("list").env("HOMEBREW_NO_AUTO_UPDATE", "1");

        if self.formulae_only {
            cmd = cmd.arg("--formulae");
        }
        if self.casks_only {
            cmd = cmd.arg("--casks");
        }
        if self.versions {
            cmd = cmd.arg("--versions");
        }
        if self.pinned {
            cmd = cmd.arg("--pinned");
        }

        cmd
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<String>, QueryError> {
    if !output.status.success() {
        let stderr = fluent_core::stderr_string(output);
        return Err(QueryError::Command(CommandError::Failed {
            args: "brew list".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let results: Vec<String> = stdout.lines().filter(|line| !line.is_empty()).map(|line| line.to_string()).collect();
    Ok(results)
}

// ── Outdated ────────────────────────────────────────────────────────

/// Builder for `brew outdated --json=v2`.
pub struct OutdatedBuilder {
    formulae_only: bool,
    casks_only: bool,
    greedy: bool,
}

impl OutdatedBuilder {
    pub(crate) fn new() -> Self {
        Self { formulae_only: false, casks_only: false, greedy: false }
    }

    /// Show only formulae.
    pub fn formulae_only(mut self) -> Self {
        self.formulae_only = true;
        self
    }

    /// Show only casks.
    pub fn casks_only(mut self) -> Self {
        self.casks_only = true;
        self
    }

    /// Include greedy casks.
    pub fn greedy(mut self) -> Self {
        self.greedy = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("outdated").arg("--json=v2").env("HOMEBREW_NO_AUTO_UPDATE", "1");

        if self.formulae_only {
            cmd = cmd.arg("--formulae");
        }
        if self.casks_only {
            cmd = cmd.arg("--casks");
        }
        if self.greedy {
            cmd = cmd.arg("--greedy");
        }

        cmd
    }
}

pub(crate) fn parse_outdated_output(output: &Output) -> Result<OutdatedResponse, QueryError> {
    if !output.status.success() {
        let stderr = fluent_core::stderr_string(output);
        return Err(QueryError::Command(CommandError::Failed {
            args: "brew outdated --json=v2".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let resp: OutdatedResponse = serde_json::from_str(&stdout)?;
    Ok(resp)
}

// ── Deps ────────────────────────────────────────────────────────────

/// Builder for `brew deps <name>`.
pub struct DepsBuilder {
    name: String,
    direct: bool,
    tree: bool,
    include_build: bool,
}

impl DepsBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), direct: false, tree: false, include_build: false }
    }

    /// Show only direct dependencies.
    pub fn direct(mut self) -> Self {
        self.direct = true;
        self
    }

    /// Show dependency tree.
    pub fn tree(mut self) -> Self {
        self.tree = true;
        self
    }

    /// Include build dependencies.
    pub fn include_build(mut self) -> Self {
        self.include_build = true;
        self
    }

    /// The formula name this builder targets.
    pub fn formula_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("brew").arg("deps").env("HOMEBREW_NO_AUTO_UPDATE", "1");

        if self.direct {
            cmd = cmd.arg("--direct");
        }
        if self.tree {
            cmd = cmd.arg("--tree");
        }
        if self.include_build {
            cmd = cmd.arg("--include-build");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }
}

pub(crate) fn parse_deps_output(output: &Output) -> Result<Vec<String>, QueryError> {
    if !output.status.success() {
        let stderr = fluent_core::stderr_string(output);
        return Err(QueryError::Command(CommandError::Failed {
            args: "brew deps".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let results: Vec<String> =
        stdout.lines().filter(|line| !line.is_empty()).map(|line| line.trim().to_string()).collect();
    Ok(results)
}
