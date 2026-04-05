use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum BranchError {
    #[error("branch '{name}' not found")]
    NotFound { name: String },
    #[error("branch '{name}' already exists")]
    AlreadyExists { name: String },
    #[error("cannot delete currently checked out branch '{name}'")]
    DeleteCurrent { name: String },
    #[error("branch '{name}' is not fully merged; use force to delete")]
    NotFullyMerged { name: String },
    #[error("invalid branch name '{name}': {reason}")]
    InvalidName { name: String, reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
