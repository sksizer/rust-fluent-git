#[derive(Debug, thiserror::Error)]
pub enum VectorizeError {
    #[error("index not found")]
    IndexNotFound,
    #[error("index already exists")]
    AlreadyExists,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
