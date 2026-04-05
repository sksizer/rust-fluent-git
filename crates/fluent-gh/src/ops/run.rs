//! Builders for `gh run` operations.

use std::fmt;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::RunError;
use crate::types::{GitHub, RunInfo, RunRerunResult};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── RunStatus enum ───────────────────────────────────────────────────

/// Filter status for listing workflow runs.
#[derive(Debug, Clone, Copy)]
pub enum RunStatus {
    Queued,
    InProgress,
    Completed,
    Failure,
    Cancelled,
}

impl fmt::Display for RunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RunStatus::Queued => "queued",
            RunStatus::InProgress => "in_progress",
            RunStatus::Completed => "completed",
            RunStatus::Failure => "failure",
            RunStatus::Cancelled => "cancelled",
        };
        write!(f, "{s}")
    }
}

// ── JSON fields ──────────────────────────────────────────────────────

const RUN_JSON_FIELDS: &str = "databaseId,displayTitle,status,conclusion,headBranch,event,createdAt,url,workflowName";

// ── Entry Point ──────────────────────────────────────────────────────

/// Entry point builder for workflow run operations.
pub struct RunBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> RunBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// List workflow runs.
    pub fn list(self) -> RunListBuilder<'a> {
        RunListBuilder::new(self.gh)
    }

    /// View a specific workflow run.
    pub fn view(self, id: impl Into<String>) -> RunViewBuilder<'a> {
        RunViewBuilder::new(self.gh, id.into())
    }

    /// Re-run a workflow run.
    pub fn rerun(self, id: impl Into<String>) -> RunRerunBuilder<'a> {
        RunRerunBuilder::new(self.gh, id.into())
    }

    /// Watch a workflow run until it completes.
    pub fn watch(self, id: impl Into<String>) -> RunWatchBuilder<'a> {
        RunWatchBuilder::new(self.gh, id.into())
    }
}

// ── List ─────────────────────────────────────────────────────────────

pub struct RunListBuilder<'a> {
    gh: &'a GitHub,
    workflow: Option<String>,
    branch: Option<String>,
    status: Option<RunStatus>,
    limit: Option<u32>,
    user: Option<String>,
}

impl<'a> RunListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, workflow: None, branch: None, status: None, limit: None, user: None }
    }

    /// Filter by workflow name or ID.
    pub fn workflow(mut self, workflow: impl Into<String>) -> Self {
        self.workflow = Some(workflow.into());
        self
    }

    /// Filter by branch.
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// Filter by run status.
    pub fn status(mut self, status: RunStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Limit the number of results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by user who triggered the run.
    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("run").arg("list").arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref workflow) = self.workflow {
            cmd = cmd.arg("--workflow").arg(workflow);
        }

        if let Some(ref branch) = self.branch {
            cmd = cmd.arg("--branch").arg(branch);
        }

        if let Some(status) = self.status {
            cmd = cmd.arg("--status").arg(status.to_string());
        }

        if let Some(limit) = self.limit {
            cmd = cmd.arg("--limit").arg(limit.to_string());
        }

        if let Some(ref user) = self.user {
            cmd = cmd.arg("--user").arg(user);
        }

        cmd = cmd.arg("--json").arg(RUN_JSON_FIELDS);

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<RunInfo>, RunError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<RunInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(RunError::NotAuthenticated);
    }

    Err(RunError::Command(CommandError::Failed {
        args: format!("run list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── View ─────────────────────────────────────────────────────────────

pub struct RunViewBuilder<'a> {
    gh: &'a GitHub,
    id: String,
}

impl<'a> RunViewBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("run")
            .arg("view")
            .arg(&self.id)
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(RUN_JSON_FIELDS)
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_view_output(output: &Output, id: &str, repo_slug: &str) -> Result<RunInfo, RunError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: RunInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(RunError::NotFound { id: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RunError::NotAuthenticated);
    }

    Err(RunError::Command(CommandError::Failed {
        args: format!("run view {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Rerun ────────────────────────────────────────────────────────────

pub struct RunRerunBuilder<'a> {
    gh: &'a GitHub,
    id: String,
    failed_only: bool,
}

impl<'a> RunRerunBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id, failed_only: false }
    }

    /// Only re-run failed jobs.
    pub fn failed_only(mut self) -> Self {
        self.failed_only = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("run").arg("rerun").arg(&self.id).arg("--repo").arg(self.gh.repo_slug());

        if self.failed_only {
            cmd = cmd.arg("--failed");
        }

        cmd
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_rerun_output(output: &Output, id: &str, repo_slug: &str) -> Result<RunRerunResult, RunError> {
    if output.status.success() {
        return Ok(RunRerunResult);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(RunError::NotFound { id: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RunError::NotAuthenticated);
    }

    Err(RunError::Command(CommandError::Failed {
        args: format!("run rerun {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Watch ────────────────────────────────────────────────────────────

pub struct RunWatchBuilder<'a> {
    gh: &'a GitHub,
    id: String,
}

impl<'a> RunWatchBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh").arg("run").arg("watch").arg(&self.id).arg("--repo").arg(self.gh.repo_slug())
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_watch_output(output: &Output, id: &str, repo_slug: &str) -> Result<(), RunError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(RunError::NotFound { id: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(RunError::NotAuthenticated);
    }

    Err(RunError::Command(CommandError::Failed {
        args: format!("run watch {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
