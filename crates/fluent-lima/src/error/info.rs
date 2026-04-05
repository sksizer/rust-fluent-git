use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum InfoError {
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for InfoError {
    fn from(e: CommandError) -> Self {
        InfoError::Command(e)
    }
}
