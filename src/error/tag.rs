use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum TagError {
    #[error("tag '{name}' already exists")]
    AlreadyExists { name: String },
    #[error("tag '{name}' not found")]
    NotFound { name: String },
    #[error("invalid tag name '{name}': {reason}")]
    InvalidName { name: String, reason: String },
    #[error("gpg signing failed for tag '{name}': {reason}")]
    SigningFailed { name: String, reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
