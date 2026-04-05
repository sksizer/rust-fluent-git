use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum IssueError {
    #[error("issue #{number} not found")]
    NotFound { number: u64 },
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error("issue #{number} is already closed")]
    AlreadyClosed { number: u64 },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse gh output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for IssueError {
    fn from(e: CommandError) -> Self {
        IssueError::Command(e)
    }
}
