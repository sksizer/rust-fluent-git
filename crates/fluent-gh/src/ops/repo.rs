//! Builders for `gh repo` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::RepoError;
use crate::types::{GitHub, RepoCreateResult, RepoInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Entry Point ────────────────────────────────────────────────────────

/// Entry point builder for repo operations.
pub struct RepoBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> RepoBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// View repository info.
    pub fn view(self) -> RepoViewBuilder<'a> {
        RepoViewBuilder::new(self.gh)
    }

    /// Clone a repository.
    pub fn clone(self) -> RepoCloneBuilder<'a> {
        RepoCloneBuilder::new(self.gh)
    }

    /// Create a new repository.
    pub fn create(self) -> RepoCreateBuilder<'a> {
        RepoCreateBuilder::new(self.gh)
    }

    /// Fork a repository.
    pub fn fork(self) -> RepoForkBuilder<'a> {
        RepoForkBuilder::new(self.gh)
    }
}

// ── View ───────────────────────────────────────────────────────────────

const REPO_VIEW_FIELDS: &str = "name,owner,description,url,isPrivate,isFork,defaultBranchRef,stargazerCount";

pub struct RepoViewBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> RepoViewBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("repo")
            .arg("view")
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(REPO_VIEW_FIELDS)
    }
}

pub(crate) fn parse_view_output(output: &Output, repo_slug: &str) -> Result<RepoInfo, RepoError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: RepoInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(RepoError::NotFound { name: repo_slug.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RepoError::NotAuthenticated);
    }

    Err(RepoError::Command(CommandError::Failed {
        args: format!("repo view --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Clone ──────────────────────────────────────────────────────────────

pub struct RepoCloneBuilder<'a> {
    gh: &'a GitHub,
    directory: Option<String>,
}

impl<'a> RepoCloneBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, directory: None }
    }

    /// Set the target directory for the clone.
    pub fn directory(mut self, dir: impl Into<String>) -> Self {
        self.directory = Some(dir.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("repo").arg("clone").arg(self.gh.repo_slug());

        if let Some(ref dir) = self.directory {
            cmd = cmd.arg(dir);
        }

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_clone_output(output: &Output, repo_slug: &str) -> Result<(), RepoError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(RepoError::NotFound { name: repo_slug.to_string() });
    }

    if lower.contains("already exists") {
        return Err(RepoError::AlreadyExists { name: repo_slug.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RepoError::NotAuthenticated);
    }

    Err(RepoError::CloneFailed { reason: stderr })
}

// ── Create ─────────────────────────────────────────────────────────────

pub struct RepoCreateBuilder<'a> {
    gh: &'a GitHub,
    description: Option<String>,
    public: bool,
    private: bool,
    clone: bool,
}

impl<'a> RepoCreateBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, description: None, public: false, private: false, clone: false }
    }

    /// Set the repository description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Make the repository public.
    pub fn public(mut self) -> Self {
        self.public = true;
        self.private = false;
        self
    }

    /// Make the repository private.
    pub fn private(mut self) -> Self {
        self.private = true;
        self.public = false;
        self
    }

    /// Clone the repository locally after creation.
    pub fn clone(mut self) -> Self {
        self.clone = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("repo").arg("create").arg(self.gh.repo_slug());

        if let Some(ref desc) = self.description {
            cmd = cmd.arg("--description").arg(desc);
        }

        if self.public {
            cmd = cmd.arg("--public");
        } else if self.private {
            cmd = cmd.arg("--private");
        }

        if self.clone {
            cmd = cmd.arg("--clone");
        }

        cmd = cmd.arg("--json").arg("url,name");

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_create_output(output: &Output, repo_slug: &str) -> Result<RepoCreateResult, RepoError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let result: RepoCreateResult = serde_json::from_str(&stdout)?;
        return Ok(result);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(RepoError::AlreadyExists { name: repo_slug.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RepoError::NotAuthenticated);
    }

    Err(RepoError::Command(CommandError::Failed {
        args: format!("repo create {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Fork ───────────────────────────────────────────────────────────────

pub struct RepoForkBuilder<'a> {
    gh: &'a GitHub,
    clone: bool,
}

impl<'a> RepoForkBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, clone: false }
    }

    /// Clone the forked repository locally.
    pub fn clone(mut self) -> Self {
        self.clone = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("repo").arg("fork").arg(self.gh.repo_slug());

        if self.clone {
            cmd = cmd.arg("--clone");
        }

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_fork_output(output: &Output, repo_slug: &str) -> Result<(), RepoError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(RepoError::NotFound { name: repo_slug.to_string() });
    }

    if lower.contains("already exists") {
        return Err(RepoError::AlreadyExists { name: repo_slug.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RepoError::NotAuthenticated);
    }

    Err(RepoError::Command(CommandError::Failed {
        args: format!("repo fork {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
