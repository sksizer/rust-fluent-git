#[derive(Debug, thiserror::Error)]
pub enum KvError {
    #[error("namespace not found")]
    NamespaceNotFound,
    #[error("key not found")]
    KeyNotFound,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
