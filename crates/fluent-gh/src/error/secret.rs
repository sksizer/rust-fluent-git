use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("secret {name:?} not found")]
    NotFound { name: String },
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse gh output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for SecretError {
    fn from(e: CommandError) -> Self {
        SecretError::Command(e)
    }
}
