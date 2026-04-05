use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum DiskError {
    #[error("disk '{name}' not found")]
    NotFound { name: String },
    #[error("disk '{name}' already exists")]
    AlreadyExists { name: String },
    #[error("disk '{name}' is in use")]
    InUse { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse limactl output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for DiskError {
    fn from(e: CommandError) -> Self {
        DiskError::Command(e)
    }
}
