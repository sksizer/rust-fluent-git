#[derive(Debug, thiserror::Error)]
pub enum WorkflowsError {
    #[error("workflow not found")]
    NotFound,
    #[error("instance not found")]
    InstanceNotFound,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
