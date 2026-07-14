#[derive(Debug, thiserror::Error)]
pub enum QueuesError {
    #[error("queue not found")]
    NotFound,
    #[error("queue already exists")]
    AlreadyExists,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
