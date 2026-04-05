use std::path::PathBuf;

use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum WorktreeError {
    #[error("worktree already exists at '{path}'")]
    AlreadyExists { path: PathBuf },
    #[error("worktree at '{path}' not found")]
    NotFound { path: PathBuf },
    #[error("branch '{branch}' is already checked out in worktree at '{path}'")]
    BranchInUse { branch: String, path: PathBuf },
    #[error("worktree at '{path}' has uncommitted changes; use force to remove")]
    DirtyWorktree { path: PathBuf },
    #[error("cannot remove main worktree")]
    CannotRemoveMain,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
