use crate::ops::{IssueBuilder, PrBuilder, ReleaseBuilder, RepoBuilder, RunBuilder, WorkflowBuilder};

/// Entry point for GitHub CLI operations.
///
/// Binds operations to an `owner/repo` pair, analogous to how
/// [`fluent_git::types::Repo`] binds git operations to a filesystem path.
#[derive(Debug, Clone)]
pub struct GitHub {
    owner: String,
    repo: String,
}

impl GitHub {
    /// Create a new handle targeting `owner/repo`.
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self { owner: owner.into(), repo: repo.into() }
    }

    /// The `owner/repo` slug used in `gh --repo` flags.
    pub fn repo_slug(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    /// Start building a `gh pr` command.
    pub fn pr(&self) -> PrBuilder<'_> {
        PrBuilder::new(self)
    }

    /// Start building a `gh issue` command.
    pub fn issue(&self) -> IssueBuilder<'_> {
        IssueBuilder::new(self)
    }

    /// Start building a `gh release` command.
    pub fn release(&self) -> ReleaseBuilder<'_> {
        ReleaseBuilder::new(self)
    }

    /// Start building a `gh repo` command.
    pub fn repo_ops(&self) -> RepoBuilder<'_> {
        RepoBuilder::new(self)
    }

    /// Start building a `gh run` command.
    pub fn run(&self) -> RunBuilder<'_> {
        RunBuilder::new(self)
    }

    /// Start building a `gh workflow` command.
    pub fn workflow(&self) -> WorkflowBuilder<'_> {
        WorkflowBuilder::new(self)
    }
}
