//! Builders for `gh workflow` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::WorkflowError;
use crate::types::{GitHub, WorkflowInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Entry Point ───────────────────────────────────────────────────────

/// Entry point builder for workflow operations.
pub struct WorkflowBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> WorkflowBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// List workflows.
    pub fn list(self) -> WorkflowListBuilder<'a> {
        WorkflowListBuilder::new(self.gh)
    }

    /// View a specific workflow.
    pub fn view(self, id_or_name: impl Into<String>) -> WorkflowViewBuilder<'a> {
        WorkflowViewBuilder::new(self.gh, id_or_name.into())
    }

    /// Run a workflow.
    pub fn run(self, id_or_name: impl Into<String>) -> WorkflowRunBuilder<'a> {
        WorkflowRunBuilder::new(self.gh, id_or_name.into())
    }

    /// Enable a workflow.
    pub fn enable(self, id_or_name: impl Into<String>) -> WorkflowEnableBuilder<'a> {
        WorkflowEnableBuilder::new(self.gh, id_or_name.into())
    }

    /// Disable a workflow.
    pub fn disable(self, id_or_name: impl Into<String>) -> WorkflowDisableBuilder<'a> {
        WorkflowDisableBuilder::new(self.gh, id_or_name.into())
    }
}

// ── List ──────────────────────────────────────────────────────────────

const WORKFLOW_LIST_FIELDS: &str = "id,name,state,path";

pub struct WorkflowListBuilder<'a> {
    gh: &'a GitHub,
    limit: Option<u32>,
}

impl<'a> WorkflowListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, limit: None }
    }

    /// Limit the number of results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("workflow").arg("list").arg("--repo").arg(self.gh.repo_slug());

        if let Some(limit) = self.limit {
            cmd = cmd.arg("--limit").arg(limit.to_string());
        }

        cmd = cmd.arg("--json").arg(WORKFLOW_LIST_FIELDS);

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<WorkflowInfo>, WorkflowError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<WorkflowInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(WorkflowError::NotAuthenticated);
    }

    Err(WorkflowError::Command(CommandError::Failed {
        args: format!("workflow list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── View ──────────────────────────────────────────────────────────────

const WORKFLOW_VIEW_FIELDS: &str = "id,name,state,path";

pub struct WorkflowViewBuilder<'a> {
    gh: &'a GitHub,
    id: String,
}

impl<'a> WorkflowViewBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("workflow")
            .arg("view")
            .arg(&self.id)
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(WORKFLOW_VIEW_FIELDS)
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_view_output(output: &Output, id: &str, repo_slug: &str) -> Result<WorkflowInfo, WorkflowError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: WorkflowInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(WorkflowError::NotFound { name: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(WorkflowError::NotAuthenticated);
    }

    Err(WorkflowError::Command(CommandError::Failed {
        args: format!("workflow view {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Run ───────────────────────────────────────────────────────────────

pub struct WorkflowRunBuilder<'a> {
    gh: &'a GitHub,
    id: String,
    ref_name: Option<String>,
    fields: Vec<String>,
}

impl<'a> WorkflowRunBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id, ref_name: None, fields: Vec::new() }
    }

    /// Set the branch or tag ref to run the workflow on.
    pub fn ref_name(mut self, ref_name: impl Into<String>) -> Self {
        self.ref_name = Some(ref_name.into());
        self
    }

    /// Add a workflow input field (key=value). Can be called multiple times.
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.fields.push(field.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd =
            ShellCommand::new("gh").arg("workflow").arg("run").arg(&self.id).arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref ref_name) = self.ref_name {
            cmd = cmd.arg("--ref").arg(ref_name);
        }

        for f in &self.fields {
            cmd = cmd.arg("--field").arg(f);
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

pub(crate) fn parse_run_output(output: &Output, id: &str, repo_slug: &str) -> Result<(), WorkflowError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(WorkflowError::NotFound { name: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(WorkflowError::NotAuthenticated);
    }

    Err(WorkflowError::Command(CommandError::Failed {
        args: format!("workflow run {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Enable ────────────────────────────────────────────────────────────

pub struct WorkflowEnableBuilder<'a> {
    gh: &'a GitHub,
    id: String,
}

impl<'a> WorkflowEnableBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh").arg("workflow").arg("enable").arg(&self.id).arg("--repo").arg(self.gh.repo_slug())
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_enable_output(output: &Output, id: &str, repo_slug: &str) -> Result<(), WorkflowError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(WorkflowError::NotFound { name: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(WorkflowError::NotAuthenticated);
    }

    Err(WorkflowError::Command(CommandError::Failed {
        args: format!("workflow enable {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Disable ───────────────────────────────────────────────────────────

pub struct WorkflowDisableBuilder<'a> {
    gh: &'a GitHub,
    id: String,
}

impl<'a> WorkflowDisableBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, id: String) -> Self {
        Self { gh, id }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh").arg("workflow").arg("disable").arg(&self.id).arg("--repo").arg(self.gh.repo_slug())
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_disable_output(output: &Output, id: &str, repo_slug: &str) -> Result<(), WorkflowError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(WorkflowError::NotFound { name: id.to_string() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(WorkflowError::NotAuthenticated);
    }

    Err(WorkflowError::Command(CommandError::Failed {
        args: format!("workflow disable {id} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
