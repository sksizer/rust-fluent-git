use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum ReleaseError {
    #[error("release with tag '{tag}' not found")]
    NotFound { tag: String },
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error("release with tag '{tag}' already exists")]
    AlreadyExists { tag: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse gh output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for ReleaseError {
    fn from(e: CommandError) -> Self {
        ReleaseError::Command(e)
    }
}
