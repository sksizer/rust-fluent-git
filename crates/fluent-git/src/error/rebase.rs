use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum RebaseError {
    #[error("rebase conflict in {files:?}")]
    Conflict { files: Vec<String> },
    #[error("cannot rebase: you have uncommitted changes")]
    DirtyWorkTree { files: Vec<String> },
    #[error("rebase target '{reference}' not found")]
    RefNotFound { reference: String },
    #[error("interactive rebase failed: {reason}")]
    InteractiveFailed { reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
