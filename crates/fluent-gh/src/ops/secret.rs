//! Builders for `gh secret` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::SecretError;
use crate::types::{GitHub, SecretInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Entry Point ───────────────────────────────────────────────────────

/// Entry point builder for secret operations.
pub struct SecretBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> SecretBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Set a secret.
    pub fn set(self, name: impl Into<String>) -> SecretSetBuilder<'a> {
        SecretSetBuilder::new(self.gh, name.into())
    }

    /// List secrets.
    pub fn list(self) -> SecretListBuilder<'a> {
        SecretListBuilder::new(self.gh)
    }

    /// Delete a secret.
    pub fn delete(self, name: impl Into<String>) -> SecretDeleteBuilder<'a> {
        SecretDeleteBuilder::new(self.gh, name.into())
    }
}

// ── Set ──────────────────────────────────────────────────────────────

pub struct SecretSetBuilder<'a> {
    gh: &'a GitHub,
    name: String,
    body: Option<String>,
}

impl<'a> SecretSetBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name, body: None }
    }

    /// Set the secret value.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("secret").arg("set").arg(&self.name).arg("--repo").arg(self.gh.repo_slug());

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

pub(crate) fn parse_set_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), SecretError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(SecretError::NotAuthenticated);
    }

    Err(SecretError::Command(CommandError::Failed {
        args: format!("secret set {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ─────────────────────────────────────────────────────────────

const SECRET_LIST_FIELDS: &str = "name,updatedAt,visibility";

pub struct SecretListBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> SecretListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("secret")
            .arg("list")
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(SECRET_LIST_FIELDS)
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<SecretInfo>, SecretError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<SecretInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(SecretError::NotAuthenticated);
    }

    Err(SecretError::Command(CommandError::Failed {
        args: format!("secret list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ───────────────────────────────────────────────────────────

pub struct SecretDeleteBuilder<'a> {
    gh: &'a GitHub,
    name: String,
}

impl<'a> SecretDeleteBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, name: String) -> Self {
        Self { gh, name }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh").arg("secret").arg("delete").arg(&self.name).arg("--repo").arg(self.gh.repo_slug())
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str, repo_slug: &str) -> Result<(), SecretError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(SecretError::NotFound { name: name.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(SecretError::NotAuthenticated);
    }

    Err(SecretError::Command(CommandError::Failed {
        args: format!("secret delete {name} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
