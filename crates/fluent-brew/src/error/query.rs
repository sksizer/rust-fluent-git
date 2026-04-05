use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("formula or cask '{name}' not found")]
    NotFound { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse brew output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for QueryError {
    fn from(e: CommandError) -> Self {
        QueryError::Command(e)
    }
}
