//! Builders for `gh label` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::LabelError;
use crate::types::{GitHub, LabelInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Entry Point ───────────────────────────────────────────────────────

/// Entry point builder for label operations.
pub struct LabelBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> LabelBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Create a new label.
    pub fn create(self, name: impl Into<String>) -> LabelCreateBuilder<'a> {
        LabelCreateBuilder::new(self.gh, name.into())
    }

    /// List labels.
    pub fn list(self) -> LabelListBuilder<'a> {
        LabelListBuilder::new(self.gh)
    }

    /// Edit an existing label.
    pub fn edit(self, name: impl Into<String>) -> LabelEditBuilder<'a> {
        LabelEditBuilder::new(self.gh, name.into())
    }

    /// Delete a label.
    pub fn delete(self, name: impl Into<String>) -> LabelDeleteBuilder<'a> {
        LabelDeleteBuilder::new(self.gh, name.into())
    }
}

// ── Create ────────────────────────────────────────────────────────────

pub struct LabelCreateBuilder<'a> {
    gh: &'a GitHub,
    name: String,
    description: Option<String>,
    color: Option<String>,
}

impl<'a> LabelCreateBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name, description: None, color: None }
    }

    /// Set the label description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the label color (hex without `#`).
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("label").arg("create").arg(&self.name).arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref description) = self.description {
            cmd = cmd.arg("--description").arg(description);
        }

        if let Some(ref color) = self.color {
            cmd = cmd.arg("--color").arg(color);
        }

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_create_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), LabelError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(LabelError::AlreadyExists { name: name.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(LabelError::NotAuthenticated);
    }

    Err(LabelError::Command(CommandError::Failed {
        args: format!("label create {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ──────────────────────────────────────────────────────────────

const LABEL_LIST_FIELDS: &str = "name,color,description";

pub struct LabelListBuilder<'a> {
    gh: &'a GitHub,
    limit: Option<u32>,
}

impl<'a> LabelListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, limit: None }
    }

    /// Limit the number of results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("label").arg("list").arg("--repo").arg(self.gh.repo_slug());

        if let Some(limit) = self.limit {
            cmd = cmd.arg("--limit").arg(limit.to_string());
        }

        cmd = cmd.arg("--json").arg(LABEL_LIST_FIELDS);

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<LabelInfo>, LabelError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<LabelInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(LabelError::NotAuthenticated);
    }

    Err(LabelError::Command(CommandError::Failed {
        args: format!("label list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Edit ──────────────────────────────────────────────────────────────

pub struct LabelEditBuilder<'a> {
    gh: &'a GitHub,
    name: String,
    new_name: Option<String>,
    description: Option<String>,
    color: Option<String>,
}

impl<'a> LabelEditBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name, new_name: None, description: None, color: None }
    }

    /// Set a new name for the label.
    pub fn new_name(mut self, new_name: impl Into<String>) -> Self {
        self.new_name = Some(new_name.into());
        self
    }

    /// Set the label description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the label color (hex without `#`).
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("label").arg("edit").arg(&self.name).arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref new_name) = self.new_name {
            cmd = cmd.arg("--name").arg(new_name);
        }

        if let Some(ref description) = self.description {
            cmd = cmd.arg("--description").arg(description);
        }

        if let Some(ref color) = self.color {
            cmd = cmd.arg("--color").arg(color);
        }

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_edit_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), LabelError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(LabelError::NotFound { name: name.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(LabelError::NotAuthenticated);
    }

    Err(LabelError::Command(CommandError::Failed {
        args: format!("label edit {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ────────────────────────────────────────────────────────────

pub struct LabelDeleteBuilder<'a> {
    gh: &'a GitHub,
    name: String,
    yes: bool,
}

impl<'a> LabelDeleteBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name, yes: false }
    }

    /// Auto-confirm the deletion.
    pub fn yes(mut self) -> Self {
        self.yes = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("label").arg("delete").arg(&self.name).arg("--repo").arg(self.gh.repo_slug());

        if self.yes {
            cmd = cmd.arg("--yes");
        }

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), LabelError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(LabelError::NotFound { name: name.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(LabelError::NotAuthenticated);
    }

    Err(LabelError::Command(CommandError::Failed {
        args: format!("label delete {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
