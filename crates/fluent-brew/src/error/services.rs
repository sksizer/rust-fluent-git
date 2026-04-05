use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("service '{name}' not found")]
    NotFound { name: String },
    #[error("service '{name}' is already running")]
    AlreadyRunning { name: String },
    #[error("service '{name}' is not running")]
    NotRunning { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse brew services output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for ServiceError {
    fn from(e: CommandError) -> Self {
        ServiceError::Command(e)
    }
}
