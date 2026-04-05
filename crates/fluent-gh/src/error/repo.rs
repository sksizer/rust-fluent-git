use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("repository '{name}' not found")]
    NotFound { name: String },
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error("repository '{name}' already exists")]
    AlreadyExists { name: String },
    #[error("clone failed: {reason}")]
    CloneFailed { reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse gh output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for RepoError {
    fn from(e: CommandError) -> Self {
        RepoError::Command(e)
    }
}
