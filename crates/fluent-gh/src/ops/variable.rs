//! Builders for `gh variable` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::VariableError;
use crate::types::{GitHub, VariableInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Entry Point ───────────────────────────────────────────────────────

/// Entry point builder for variable operations.
pub struct VariableBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> VariableBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Set a variable.
    pub fn set(self, name: impl Into<String>) -> VariableSetBuilder<'a> {
        VariableSetBuilder::new(self.gh, name.into())
    }

    /// List variables.
    pub fn list(self) -> VariableListBuilder<'a> {
        VariableListBuilder::new(self.gh)
    }

    /// Delete a variable.
    pub fn delete(self, name: impl Into<String>) -> VariableDeleteBuilder<'a> {
        VariableDeleteBuilder::new(self.gh, name.into())
    }

    /// Get a specific variable.
    pub fn get(self, name: impl Into<String>) -> VariableGetBuilder<'a> {
        VariableGetBuilder::new(self.gh, name.into())
    }
}

// ── Set ──────────────────────────────────────────────────────────────

pub struct VariableSetBuilder<'a> {
    gh: &'a GitHub,
    name: String,
    body: Option<String>,
}

impl<'a> VariableSetBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name, body: None }
    }

    /// Set the variable value.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("variable").arg("set").arg(&self.name).arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref body) = self.body {
            cmd = cmd.arg("--body").arg(body);
        }

        cmd
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_set_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), VariableError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(VariableError::NotAuthenticated);
    }

    Err(VariableError::Command(CommandError::Failed {
        args: format!("variable set {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ─────────────────────────────────────────────────────────────

const VARIABLE_LIST_FIELDS: &str = "name,value,updatedAt,visibility";

pub struct VariableListBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> VariableListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("variable")
            .arg("list")
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(VARIABLE_LIST_FIELDS)
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<VariableInfo>, VariableError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<VariableInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(VariableError::NotAuthenticated);
    }

    Err(VariableError::Command(CommandError::Failed {
        args: format!("variable list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ───────────────────────────────────────────────────────────

pub struct VariableDeleteBuilder<'a> {
    gh: &'a GitHub,
    name: String,
}

impl<'a> VariableDeleteBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh").arg("variable").arg("delete").arg(&self.name).arg("--repo").arg(self.gh.repo_slug())
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), VariableError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(VariableError::NotFound { name: name.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(VariableError::NotAuthenticated);
    }

    Err(VariableError::Command(CommandError::Failed {
        args: format!("variable delete {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Get ──────────────────────────────────────────────────────────────

const VARIABLE_GET_FIELDS: &str = "name,value,updatedAt,visibility";

pub struct VariableGetBuilder<'a> {
    gh: &'a GitHub,
    name: String,
}

impl<'a> VariableGetBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("variable")
            .arg("get")
            .arg(&self.name)
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(VARIABLE_GET_FIELDS)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_get_output(output: &Output, name: &str, repo_slug: &str) -> Result<VariableInfo, VariableError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: VariableInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(VariableError::NotFound { name: name.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(VariableError::NotAuthenticated);
    }

    Err(VariableError::Command(CommandError::Failed {
        args: format!("variable get {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
