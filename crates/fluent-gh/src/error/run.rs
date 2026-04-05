use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("workflow run '{id}' not found")]
    NotFound { id: String },
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse gh output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for RunError {
    fn from(e: CommandError) -> Self {
        RunError::Command(e)
    }
}
