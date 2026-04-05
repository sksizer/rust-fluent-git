use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("workflow '{name}' not found")]
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

impl From<CommandError> for WorkflowError {
    fn from(e: CommandError) -> Self {
        WorkflowError::Command(e)
    }
}
