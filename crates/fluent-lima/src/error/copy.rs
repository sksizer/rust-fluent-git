use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum CopyError {
    #[error("instance '{name}' not found")]
    NotFound { name: String },
    #[error("instance '{name}' is not running")]
    NotRunning { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for CopyError {
    fn from(e: CommandError) -> Self {
        CopyError::Command(e)
    }
}
