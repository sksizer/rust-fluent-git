use std::path::PathBuf;

use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum OpenError {
    #[error("not a git repository (or any parent): {path}")]
    NotARepo { path: PathBuf },
    #[error("repository at {path} is not accessible: {reason}")]
    NotAccessible { path: PathBuf, reason: String },
    #[error("repository at {path} appears corrupt: {reason}")]
    CorruptRepo { path: PathBuf, reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
