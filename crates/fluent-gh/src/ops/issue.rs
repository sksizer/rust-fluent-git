//! Builders for `gh issue` operations.

use std::fmt;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::IssueError;
use crate::types::{GitHub, IssueCreateResult, IssueInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── Issue State ───────────────────────────────────────────────────────

/// Filter state for issue list queries.
#[derive(Debug, Clone, Copy, Default)]
pub enum IssueState {
    #[default]
    Open,
    Closed,
    All,
}

impl fmt::Display for IssueState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueState::Open => write!(f, "open"),
            IssueState::Closed => write!(f, "closed"),
            IssueState::All => write!(f, "all"),
        }
    }
}

// ── Entry Point ───────────────────────────────────────────────────────

/// Entry point builder for issue operations.
pub struct IssueBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> IssueBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Create a new issue.
    pub fn create(self) -> IssueCreateBuilder<'a> {
        IssueCreateBuilder::new(self.gh)
    }

    /// List issues.
    pub fn list(self) -> IssueListBuilder<'a> {
        IssueListBuilder::new(self.gh)
    }

    /// View a specific issue.
    pub fn view(self, number: u64) -> IssueViewBuilder<'a> {
        IssueViewBuilder::new(self.gh, number)
    }

    /// Close an issue.
    pub fn close(self, number: u64) -> IssueCloseBuilder<'a> {
        IssueCloseBuilder::new(self.gh, number)
    }

    /// Add a comment to an issue.
    pub fn comment(self, number: u64) -> IssueCommentBuilder<'a> {
        IssueCommentBuilder::new(self.gh, number)
    }
}

// ── JSON Fields ───────────────────────────────────────────────────────

const ISSUE_JSON_FIELDS: &str = "number,title,state,author,url,createdAt,labels,assignees";

// ── Create ────────────────────────────────────────────────────────────

pub struct IssueCreateBuilder<'a> {
    gh: &'a GitHub,
    title: Option<String>,
    body: Option<String>,
    labels: Vec<String>,
    assignees: Vec<String>,
}

impl<'a> IssueCreateBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, title: None, body: None, labels: Vec::new(), assignees: Vec::new() }
    }

    /// Set the issue title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the issue body.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Add a label (can be called multiple times).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    /// Add an assignee (can be called multiple times).
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignees.push(assignee.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("issue").arg("create").arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref title) = self.title {
            cmd = cmd.arg("--title").arg(title);
        }

        if let Some(ref body) = self.body {
            cmd = cmd.arg("--body").arg(body);
        }

        for label in &self.labels {
            cmd = cmd.arg("--label").arg(label);
        }

        for assignee in &self.assignees {
            cmd = cmd.arg("--assignee").arg(assignee);
        }

        cmd = cmd.arg("--json").arg("number,url,title");

        cmd
    }
}

pub(crate) fn parse_create_output(output: &Output) -> Result<IssueCreateResult, IssueError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let result: IssueCreateResult = serde_json::from_str(&stdout)?;
        return Ok(result);
    }

    classify_issue_error(output, "issue create")
}

// ── List ──────────────────────────────────────────────────────────────

pub struct IssueListBuilder<'a> {
    gh: &'a GitHub,
    state: IssueState,
    author: Option<String>,
    label: Option<String>,
    assignee: Option<String>,
    limit: Option<u32>,
}

impl<'a> IssueListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, state: IssueState::default(), author: None, label: None, assignee: None, limit: None }
    }

    /// Filter by issue state.
    pub fn state(mut self, state: IssueState) -> Self {
        self.state = state;
        self
    }

    /// Filter by author.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Filter by label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Filter by assignee.
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// Limit the number of results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh")
            .arg("issue")
            .arg("list")
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--state")
            .arg(self.state.to_string());

        if let Some(ref author) = self.author {
            cmd = cmd.arg("--author").arg(author);
        }

        if let Some(ref label) = self.label {
            cmd = cmd.arg("--label").arg(label);
        }

        if let Some(ref assignee) = self.assignee {
            cmd = cmd.arg("--assignee").arg(assignee);
        }

        if let Some(limit) = self.limit {
            cmd = cmd.arg("--limit").arg(limit.to_string());
        }

        cmd = cmd.arg("--json").arg(ISSUE_JSON_FIELDS);

        cmd
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<IssueInfo>, IssueError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let issues: Vec<IssueInfo> = serde_json::from_str(&stdout)?;
        return Ok(issues);
    }

    classify_issue_error(output, "issue list")
}

// ── View ──────────────────────────────────────────────────────────────

pub struct IssueViewBuilder<'a> {
    gh: &'a GitHub,
    number: u64,
}

impl<'a> IssueViewBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, number: u64) -> Self {
        Self { gh, number }
    }

    pub(crate) fn number(&self) -> u64 {
        self.number
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("issue")
            .arg("view")
            .arg(self.number.to_string())
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(ISSUE_JSON_FIELDS)
    }
}

pub(crate) fn parse_view_output(output: &Output, number: u64) -> Result<IssueInfo, IssueError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: IssueInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(IssueError::NotFound { number });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(IssueError::NotAuthenticated);
    }

    Err(IssueError::Command(CommandError::Failed {
        args: format!("issue view {number}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Close ─────────────────────────────────────────────────────────────

pub struct IssueCloseBuilder<'a> {
    gh: &'a GitHub,
    number: u64,
}

impl<'a> IssueCloseBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, number: u64) -> Self {
        Self { gh, number }
    }

    pub(crate) fn number(&self) -> u64 {
        self.number
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("issue")
            .arg("close")
            .arg(self.number.to_string())
            .arg("--repo")
            .arg(self.gh.repo_slug())
    }
}

pub(crate) fn parse_close_output(output: &Output, number: u64) -> Result<(), IssueError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(IssueError::NotFound { number });
    }

    if lower.contains("already closed") || lower.contains("is closed") {
        return Err(IssueError::AlreadyClosed { number });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(IssueError::NotAuthenticated);
    }

    Err(IssueError::Command(CommandError::Failed {
        args: format!("issue close {number}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Comment ───────────────────────────────────────────────────────────

pub struct IssueCommentBuilder<'a> {
    gh: &'a GitHub,
    number: u64,
    body: Option<String>,
}

impl<'a> IssueCommentBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, number: u64) -> Self {
        Self { gh, number, body: None }
    }

    /// Set the comment body.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub(crate) fn number(&self) -> u64 {
        self.number
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh")
            .arg("issue")
            .arg("comment")
            .arg(self.number.to_string())
            .arg("--repo")
            .arg(self.gh.repo_slug());

        if let Some(ref body) = self.body {
            cmd = cmd.arg("--body").arg(body);
        }

        cmd
    }
}

pub(crate) fn parse_comment_output(output: &Output, number: u64) -> Result<(), IssueError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(IssueError::NotFound { number });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(IssueError::NotAuthenticated);
    }

    Err(IssueError::Command(CommandError::Failed {
        args: format!("issue comment {number}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Shared Error Classification ───────────────────────────────────────

fn classify_issue_error<T>(output: &Output, command_desc: &str) -> Result<T, IssueError> {
    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(IssueError::NotAuthenticated);
    }

    Err(IssueError::Command(CommandError::Failed {
        args: command_desc.to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
