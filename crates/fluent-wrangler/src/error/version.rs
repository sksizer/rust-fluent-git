#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("version not found")]
    NotFound,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
