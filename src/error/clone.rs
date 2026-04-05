use std::path::PathBuf;

use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum CloneError {
    #[error("destination path '{path}' already exists and is not empty")]
    DestinationExists { path: PathBuf },
    #[error("repository '{url}' not found or not accessible")]
    RepoNotFound { url: String },
    #[error("authentication failed for '{url}'")]
    AuthFailed { url: String },
    #[error("remote branch '{branch}' not found in '{url}'")]
    BranchNotFound { url: String, branch: String },
    #[error("network error while cloning '{url}': {reason}")]
    Network { url: String, reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
