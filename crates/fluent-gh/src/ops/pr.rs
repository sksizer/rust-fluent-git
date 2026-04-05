//! Builders for `gh pr` operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::PrError;
use crate::types::{GitHub, PrCreateResult, PrInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── PrState enum ──────────────────────────────────────────────────────

/// Filter state for listing pull requests.
#[derive(Debug, Clone, Copy)]
pub enum PrState {
    Open,
    Closed,
    Merged,
    All,
}

impl PrState {
    fn as_str(self) -> &'static str {
        match self {
            PrState::Open => "open",
            PrState::Closed => "closed",
            PrState::Merged => "merged",
            PrState::All => "all",
        }
    }
}

// ── Entry Point ───────────────────────────────────────────────────────

/// Entry point builder for PR operations.
pub struct PrBuilder<'a> {
    gh: &'a GitHub,
}

impl<'a> PrBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh }
    }

    /// Create a new pull request.
    pub fn create(self) -> PrCreateBuilder<'a> {
        PrCreateBuilder::new(self.gh)
    }

    /// List pull requests.
    pub fn list(self) -> PrListBuilder<'a> {
        PrListBuilder::new(self.gh)
    }

    /// View a specific pull request.
    pub fn view(self, number: u64) -> PrViewBuilder<'a> {
        PrViewBuilder::new(self.gh, number)
    }

    /// Merge a pull request.
    pub fn merge(self, number: u64) -> PrMergeBuilder<'a> {
        PrMergeBuilder::new(self.gh, number)
    }

    /// Close a pull request.
    pub fn close(self, number: u64) -> PrCloseBuilder<'a> {
        PrCloseBuilder::new(self.gh, number)
    }

    /// Check out a pull request locally.
    pub fn checkout(self, number_or_branch: impl Into<String>) -> PrCheckoutBuilder<'a> {
        PrCheckoutBuilder::new(self.gh, number_or_branch.into())
    }
}

// ── Create ────────────────────────────────────────────────────────────

pub struct PrCreateBuilder<'a> {
    gh: &'a GitHub,
    title: Option<String>,
    body: Option<String>,
    base: Option<String>,
    head: Option<String>,
    draft: bool,
    labels: Vec<String>,
    reviewers: Vec<String>,
    assignee: Option<String>,
}

impl<'a> PrCreateBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self {
            gh,
            title: None,
            body: None,
            base: None,
            head: None,
            draft: false,
            labels: Vec::new(),
            reviewers: Vec::new(),
            assignee: None,
        }
    }

    /// Set the PR title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the PR body.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set the base branch to merge into.
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    /// Set the head branch containing changes.
    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    /// Mark the PR as a draft.
    pub fn draft(mut self) -> Self {
        self.draft = true;
        self
    }

    /// Add a label (can be called multiple times).
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    /// Add a reviewer (can be called multiple times).
    pub fn reviewer(mut self, reviewer: impl Into<String>) -> Self {
        self.reviewers.push(reviewer.into());
        self
    }

    /// Set the assignee.
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("pr").arg("create").arg("--repo").arg(self.gh.repo_slug());

        if let Some(ref title) = self.title {
            cmd = cmd.arg("--title").arg(title);
        }

        if let Some(ref body) = self.body {
            cmd = cmd.arg("--body").arg(body);
        }

        if let Some(ref base) = self.base {
            cmd = cmd.arg("--base").arg(base);
        }

        if let Some(ref head) = self.head {
            cmd = cmd.arg("--head").arg(head);
        }

        if self.draft {
            cmd = cmd.arg("--draft");
        }

        for label in &self.labels {
            cmd = cmd.arg("--label").arg(label);
        }

        for reviewer in &self.reviewers {
            cmd = cmd.arg("--reviewer").arg(reviewer);
        }

        if let Some(ref assignee) = self.assignee {
            cmd = cmd.arg("--assignee").arg(assignee);
        }

        cmd = cmd.arg("--json").arg("number,url,title");

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_create_output(output: &Output, repo_slug: &str) -> Result<PrCreateResult, PrError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let result: PrCreateResult = serde_json::from_str(&stdout)?;
        return Ok(result);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(PrError::AlreadyExists { head: String::new(), base: String::new() });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(PrError::NotAuthenticated);
    }

    Err(PrError::Command(CommandError::Failed {
        args: format!("pr create --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ──────────────────────────────────────────────────────────────

const PR_LIST_FIELDS: &str = "number,title,state,author,url,headRefName,baseRefName,isDraft,createdAt,labels";

pub struct PrListBuilder<'a> {
    gh: &'a GitHub,
    state: Option<PrState>,
    author: Option<String>,
    label: Option<String>,
    limit: Option<u32>,
    base: Option<String>,
    head: Option<String>,
}

impl<'a> PrListBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub) -> Self {
        Self { gh, state: None, author: None, label: None, limit: None, base: None, head: None }
    }

    /// Filter by PR state.
    pub fn state(mut self, state: PrState) -> Self {
        self.state = Some(state);
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

    /// Limit the number of results.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by base branch.
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    /// Filter by head branch.
    pub fn head(mut self, head: impl Into<String>) -> Self {
        self.head = Some(head.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("pr").arg("list").arg("--repo").arg(self.gh.repo_slug());

        if let Some(state) = self.state {
            cmd = cmd.arg("--state").arg(state.as_str());
        }

        if let Some(ref author) = self.author {
            cmd = cmd.arg("--author").arg(author);
        }

        if let Some(ref label) = self.label {
            cmd = cmd.arg("--label").arg(label);
        }

        if let Some(limit) = self.limit {
            cmd = cmd.arg("--limit").arg(limit.to_string());
        }

        if let Some(ref base) = self.base {
            cmd = cmd.arg("--base").arg(base);
        }

        if let Some(ref head) = self.head {
            cmd = cmd.arg("--head").arg(head);
        }

        cmd = cmd.arg("--json").arg(PR_LIST_FIELDS);

        cmd
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_list_output(output: &Output, repo_slug: &str) -> Result<Vec<PrInfo>, PrError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let items: Vec<PrInfo> = serde_json::from_str(&stdout)?;
        return Ok(items);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(PrError::NotAuthenticated);
    }

    Err(PrError::Command(CommandError::Failed {
        args: format!("pr list --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── View ──────────────────────────────────────────────────────────────

const PR_VIEW_FIELDS: &str = "number,title,state,author,url,headRefName,baseRefName,isDraft,createdAt,labels";

pub struct PrViewBuilder<'a> {
    gh: &'a GitHub,
    number: u64,
}

impl<'a> PrViewBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, number: u64) -> Self {
        Self { gh, number }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("pr")
            .arg("view")
            .arg(self.number.to_string())
            .arg("--repo")
            .arg(self.gh.repo_slug())
            .arg("--json")
            .arg(PR_VIEW_FIELDS)
    }

    pub(crate) fn number(&self) -> u64 {
        self.number
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_view_output(output: &Output, number: u64, repo_slug: &str) -> Result<PrInfo, PrError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: PrInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(PrError::NotFound { number });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(PrError::NotAuthenticated);
    }

    Err(PrError::Command(CommandError::Failed {
        args: format!("pr view {number} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Merge ─────────────────────────────────────────────────────────────

pub struct PrMergeBuilder<'a> {
    gh: &'a GitHub,
    number: u64,
    squash: bool,
    rebase: bool,
    merge: bool,
    delete_branch: bool,
    admin: bool,
}

impl<'a> PrMergeBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, number: u64) -> Self {
        Self { gh, number, squash: false, rebase: false, merge: false, delete_branch: false, admin: false }
    }

    /// Use squash merge.
    pub fn squash(mut self) -> Self {
        self.squash = true;
        self.rebase = false;
        self.merge = false;
        self
    }

    /// Use rebase merge.
    pub fn rebase(mut self) -> Self {
        self.rebase = true;
        self.squash = false;
        self.merge = false;
        self
    }

    /// Use merge commit.
    pub fn merge(mut self) -> Self {
        self.merge = true;
        self.squash = false;
        self.rebase = false;
        self
    }

    /// Delete the branch after merging.
    pub fn delete_branch(mut self) -> Self {
        self.delete_branch = true;
        self
    }

    /// Merge even if requirements are not met (admin override).
    pub fn admin(mut self) -> Self {
        self.admin = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh")
            .arg("pr")
            .arg("merge")
            .arg(self.number.to_string())
            .arg("--repo")
            .arg(self.gh.repo_slug());

        if self.squash {
            cmd = cmd.arg("--squash");
        } else if self.rebase {
            cmd = cmd.arg("--rebase");
        } else if self.merge {
            cmd = cmd.arg("--merge");
        }

        if self.delete_branch {
            cmd = cmd.arg("--delete-branch");
        }

        if self.admin {
            cmd = cmd.arg("--admin");
        }

        cmd
    }

    pub(crate) fn number(&self) -> u64 {
        self.number
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_merge_output(output: &Output, number: u64, repo_slug: &str) -> Result<(), PrError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(PrError::NotFound { number });
    }

    if lower.contains("merge conflict") || lower.contains("not mergeable") {
        return Err(PrError::MergeConflict { reason: stderr.clone() });
    }

    if lower.contains("check") && lower.contains("fail") {
        return Err(PrError::ChecksFailed);
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(PrError::NotAuthenticated);
    }

    Err(PrError::Command(CommandError::Failed {
        args: format!("pr merge {number} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Close ─────────────────────────────────────────────────────────────

pub struct PrCloseBuilder<'a> {
    gh: &'a GitHub,
    number: u64,
}

impl<'a> PrCloseBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, number: u64) -> Self {
        Self { gh, number }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh")
            .arg("pr")
            .arg("close")
            .arg(self.number.to_string())
            .arg("--repo")
            .arg(self.gh.repo_slug())
    }

    pub(crate) fn number(&self) -> u64 {
        self.number
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_close_output(output: &Output, number: u64, repo_slug: &str) -> Result<(), PrError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(PrError::NotFound { number });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(PrError::NotAuthenticated);
    }

    Err(PrError::Command(CommandError::Failed {
        args: format!("pr close {number} --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Checkout ──────────────────────────────────────────────────────────

pub struct PrCheckoutBuilder<'a> {
    gh: &'a GitHub,
    pr: String,
}

impl<'a> PrCheckoutBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, pr: String) -> Self {
        Self { gh, pr }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("gh").arg("pr").arg("checkout").arg(&self.pr).arg("--repo").arg(self.gh.repo_slug())
    }

    pub(crate) fn repo_slug(&self) -> String {
        self.gh.repo_slug()
    }
}

pub(crate) fn parse_checkout_output(output: &Output, repo_slug: &str) -> Result<(), PrError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("could not resolve") {
        return Err(PrError::NotFound { number: 0 });
    }

    if lower.contains("auth") || lower.contains("login") {
        return Err(PrError::NotAuthenticated);
    }

    Err(PrError::Command(CommandError::Failed {
        args: format!("pr checkout --repo {repo_slug}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
