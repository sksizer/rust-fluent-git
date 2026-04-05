use std::path::{Path, PathBuf};

use crate::ops::{
    AddBuilder, BranchBuilder, CheckoutBuilder, CherryPickBuilder, CleanBuilder, CommitBuilder,
    ConfigBuilder, DiffBuilder, LogBuilder, MergeBuilder, RebaseBuilder, RemoteBuilder,
    ResetBuilder, RevParseBuilder, StashBuilder, StatusBuilder, TagBuilder, WorktreeBuilder,
};

/// Handle to a git repository. Proof that setup succeeded.
/// All repo operations hang off this type.
#[derive(Debug, Clone)]
pub struct Repo {
    path: PathBuf,
}

impl Repo {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Start building a `git add` command.
    pub fn add(&self) -> AddBuilder<'_> {
        AddBuilder::new(&self.path)
    }

    /// Start building a `git commit` command.
    pub fn commit(&self) -> CommitBuilder<'_> {
        CommitBuilder::new(&self.path)
    }

    /// Start building a `git branch` command.
    pub fn branch(&self) -> BranchBuilder<'_> {
        BranchBuilder::new(&self.path)
    }

    /// Start building a `git checkout` command.
    pub fn checkout(&self) -> CheckoutBuilder<'_> {
        CheckoutBuilder::new(&self.path)
    }

    /// Start building a `git config` command.
    pub fn config(&self) -> ConfigBuilder<'_> {
        ConfigBuilder::new(&self.path)
    }

    /// Start building a `git status` command.
    pub fn status(&self) -> StatusBuilder<'_> {
        StatusBuilder::new(&self.path)
    }

    /// Start building a `git log` command.
    pub fn log(&self) -> LogBuilder<'_> {
        LogBuilder::new(&self.path)
    }

    /// Start building a `git diff` command.
    pub fn diff(&self) -> DiffBuilder<'_> {
        DiffBuilder::new(&self.path)
    }

    /// Start building a `git worktree` command.
    pub fn worktree(&self) -> WorktreeBuilder<'_> {
        WorktreeBuilder::new(&self.path)
    }

    /// Start building a `git stash` command.
    pub fn stash(&self) -> StashBuilder<'_> {
        StashBuilder::new(&self.path)
    }

    /// Start building a `git remote` command.
    pub fn remote(&self) -> RemoteBuilder<'_> {
        RemoteBuilder::new(&self.path)
    }

    /// Start building a `git tag` command.
    pub fn tag(&self) -> TagBuilder<'_> {
        TagBuilder::new(&self.path)
    }

    /// Start building a `git reset` command.
    pub fn reset(&self) -> ResetBuilder<'_> {
        ResetBuilder::new(&self.path)
    }

    /// Start building a `git merge` command.
    pub fn merge(&self) -> MergeBuilder<'_> {
        MergeBuilder::new(&self.path)
    }

    /// Start building a `git rebase` command.
    pub fn rebase(&self) -> RebaseBuilder<'_> {
        RebaseBuilder::new(&self.path)
    }

    /// Start building a `git cherry-pick` command.
    pub fn cherry_pick(&self) -> CherryPickBuilder<'_> {
        CherryPickBuilder::new(&self.path)
    }

    /// Start building a `git clean` command.
    pub fn clean(&self) -> CleanBuilder<'_> {
        CleanBuilder::new(&self.path)
    }

    /// Start building a `git rev-parse` command for the given ref.
    pub fn rev_parse(&self, reference: impl Into<String>) -> RevParseBuilder<'_> {
        RevParseBuilder::new(&self.path, reference)
    }
}
