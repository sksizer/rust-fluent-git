//! Builders for `gh release` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::ReleaseError;
use crate::types::{GitHub, ReleaseCreateResult, ReleaseInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── JSON field lists ─────────────────────────────────────────────────

const RELEASE_FIELDS: &str = "tagName,name,isDraft,isPrerelease,createdAt,url,author,assets";
const RELEASE_CREATE_FIELDS: &str = "tagName,url,name";

// ── Entry Point ──────────────────────────────────────────────────────

/// Entry point builder for release operations.
pub struct ReleaseBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> ReleaseBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Create a new release.
    pub fn create(self, tag: impl Into<String>) -> ReleaseCreateBuilder<'a> {
        ReleaseCreateBuilder::new(self.gh, tag.into())
    }

    /// List releases.
    pub fn list(self) -> ReleaseListBuilder<'a> {
        ReleaseListBuilder::new(self.gh)
    }

    /// View a specific release.
    pub fn view(self, tag: impl Into<String>) -> ReleaseViewBuilder<'a> {
        ReleaseViewBuilder::new(self.gh, tag.into())
    }

    /// Delete a release.
    pub fn delete(self, tag: impl Into<String>) -> ReleaseDeleteBuilder<'a> {
        ReleaseDeleteBuilder::new(self.gh, tag.into())
    }
}

// ── Create ───────────────────────────────────────────────────────────

pub struct ReleaseCreateBuilder<'a> {
    gh: &'a GitHub,
    tag: String,
    title: Option<String>,
    notes: Option<String>,
    draft: bool,
    prerelease: bool,
    target: Option<String>,
    files: Vec<String>,
}

impl<'a> ReleaseCreateBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, tag: String) -> Self {
        Self { gh, tag, title: None, notes: None, draft: false, prerelease: false, target: None, files: Vec::new() }
    }

    /// Set the release title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the release notes.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Mark the release as a draft.
    pub fn draft(mut self) -> Self {
        self.draft = true;
        self
    }

    /// Mark the release as a prerelease.
    pub fn prerelease(mut self) -> Self {
        self.prerelease = true;
        self
    }

    /// Set the target commitish (branch or SHA).
    pub fn target(mut self, commitish: impl Into<String>) -> Self {
        self.target = Some(commitish.into());
        self
    }

    /// Add files to attach to the release.
    pub fn files(mut self, files: Vec<String>) -> Self {
        self.files = files;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("release").arg("create").arg(&self.tag).arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref title) = self.title {
            cmd = cmd.arg("--title").arg(title);
        }

        if let Some(ref notes) = self.notes {
            cmd = cmd.arg("--notes").arg(notes);
        }

        if self.draft {
            cmd = cmd.arg("--draft");
        }

        if self.prerelease {
            cmd = cmd.arg("--prerelease");
        }

        if let Some(ref target) = self.target {
            cmd = cmd.arg("--target").arg(target);
        }

        for file in &self.files {
            cmd = cmd.arg(file);
        }

        cmd = cmd.arg("--json").arg(RELEASE_CREATE_FIELDS);

        cmd
    }

    pub(crate) fn tag(&self) -> &str {
        &self.tag
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_create_output(
    output: &Output,
    tag: &str,
    repo_slug: &str,
) -> Result<ReleaseCreateResult, ReleaseError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let result: ReleaseCreateResult = serde_json::from_str(&stdout)?;
        return Ok(result);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(ReleaseError::AlreadyExists { tag: tag.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(ReleaseError::NotAuthenticated);
    }

    Err(ReleaseError::Command(CommandError::Failed {
        args: format!("release create {tag} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ─────────────────────────────────────────────────────────────

pub struct ReleaseListBuilder<'a> {
    gh: &'a GitHub,
    limit: Option<u32>,
}

impl<'a> ReleaseListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, limit: None }
    }

    /// Limit the number of results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("release").arg("list").arg("--repo").arg(self.gh.repo_slug());

        if let Some(limit) = self.limit {
            cmd = cmd.arg("--limit").arg(limit.to_string());
        }

        cmd = cmd.arg("--json").arg(RELEASE_FIELDS);

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<ReleaseInfo>, ReleaseError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<ReleaseInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(ReleaseError::NotAuthenticated);
    }

    Err(ReleaseError::Command(CommandError::Failed {
        args: format!("release list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── View ─────────────────────────────────────────────────────────────

pub struct ReleaseViewBuilder<'a> {
    gh: &'a GitHub,
    tag: String,
}

impl<'a> ReleaseViewBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, tag: String) -> Self {
        Self { gh, tag }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("release")
            .arg("view")
            .arg(&self.tag)
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(RELEASE_FIELDS)
    }

    pub(crate) fn tag(&self) -> &str {
        &self.tag
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_view_output(output: &Output, tag: &str, repo_slug: &str) -> Result<ReleaseInfo, ReleaseError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: ReleaseInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(ReleaseError::NotFound { tag: tag.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(ReleaseError::NotAuthenticated);
    }

    Err(ReleaseError::Command(CommandError::Failed {
        args: format!("release view {tag} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ───────────────────────────────────────────────────────────

pub struct ReleaseDeleteBuilder<'a> {
    gh: &'a GitHub,
    tag: String,
    yes: bool,
}

impl<'a> ReleaseDeleteBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, tag: String) -> Self {
        Self { gh, tag, yes: false }
    }

    /// Auto-confirm the deletion (skip interactive prompt).
    pub fn yes(mut self) -> Self {
        self.yes = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("release").arg("delete").arg(&self.tag).arg("--repo").arg(self.gh.repo_slug());

        if self.yes {
            cmd = cmd.arg("--yes");
        }

        cmd
    }

    pub(crate) fn tag(&self) -> &str {
        &self.tag
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_delete_output(output: &Output, tag: &str, repo_slug: &str) -> Result<(), ReleaseError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(ReleaseError::NotFound { tag: tag.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(ReleaseError::NotAuthenticated);
    }

    Err(ReleaseError::Command(CommandError::Failed {
        args: format!("release delete {tag} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
