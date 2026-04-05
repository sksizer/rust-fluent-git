use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum MergeError {
    #[error("merge conflict in {files:?}")]
    Conflict { files: Vec<String> },
    #[error("cannot merge: you have uncommitted changes in {files:?}")]
    DirtyWorkTree { files: Vec<String> },
    #[error("merge ref '{reference}' not found")]
    RefNotFound { reference: String },
    #[error("cannot merge a branch into itself: '{name}'")]
    SelfMerge { name: String },
    #[error("merge aborted by user")]
    Aborted,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
