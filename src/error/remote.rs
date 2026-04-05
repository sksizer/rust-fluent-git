use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum RemoteError {
    #[error("remote '{name}' not found")]
    NotFound { name: String },
    #[error("remote '{name}' already exists")]
    AlreadyExists { name: String },
    #[error("invalid remote url '{url}': {reason}")]
    InvalidUrl { url: String, reason: String },
    #[error("authentication failed for remote '{name}' ({url})")]
    AuthFailed { name: String, url: String },
    #[error("push to '{name}' rejected: {reason}")]
    PushRejected { name: String, reason: String },
    #[error("fetch from '{name}' failed: {reason}")]
    FetchFailed { name: String, reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
