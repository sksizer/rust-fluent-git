#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
