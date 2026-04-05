use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum InstanceError {
    #[error("instance '{name}' not found")]
    NotFound { name: String },
    #[error("instance '{name}' already exists")]
    AlreadyExists { name: String },
    #[error("instance '{name}' is already running")]
    AlreadyRunning { name: String },
    #[error("instance '{name}' is already stopped")]
    AlreadyStopped { name: String },
    #[error("instance '{name}' is protected")]
    Protected { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse limactl output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for InstanceError {
    fn from(e: CommandError) -> Self {
        InstanceError::Command(e)
    }
}
