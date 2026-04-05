use std::path::PathBuf;

use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("repository already exists at {path}")]
    AlreadyExists { path: PathBuf },
    #[error("permission denied creating repository at {path}")]
    PermissionDenied { path: PathBuf },
    #[error("invalid initial branch name '{name}': {reason}")]
    InvalidBranchName { name: String, reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
