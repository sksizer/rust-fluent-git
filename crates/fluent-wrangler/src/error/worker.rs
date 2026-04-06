#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("worker not found")]
    NotFound,
    #[error("worker already exists")]
    AlreadyExists,
    #[error("deploy failed")]
    DeployFailed,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
