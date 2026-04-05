use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum PrError {
    #[error("pull request #{number} not found")]
    NotFound { number: u64 },
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error("a pull request already exists for {head} into {base}")]
    AlreadyExists { head: String, base: String },
    #[error("merge conflict: {reason}")]
    MergeConflict { reason: String },
    #[error("checks failed on pull request")]
    ChecksFailed,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse gh output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for PrError {
    fn from(e: CommandError) -> Self {
        PrError::Command(e)
    }
}
