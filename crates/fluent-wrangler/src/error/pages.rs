#[derive(Debug, thiserror::Error)]
pub enum PagesError {
    #[error("project not found")]
    ProjectNotFound,
    #[error("deploy failed")]
    DeployFailed,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
