use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum CherryPickError {
    #[error("cherry-pick conflict in {files:?}")]
    Conflict { files: Vec<String> },
    #[error("commit '{sha}' not found")]
    CommitNotFound { sha: String },
    #[error("cannot cherry-pick: you have uncommitted changes")]
    DirtyWorkTree { files: Vec<String> },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
