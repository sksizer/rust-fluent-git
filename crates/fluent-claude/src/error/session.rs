use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("session '{session_id}' not found")]
    NotFound { session_id: String },
    #[error("not authenticated; run `claude auth login`")]
    NotAuthenticated,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for SessionError {
    fn from(e: CommandError) -> Self {
        SessionError::Command(e)
    }
}
