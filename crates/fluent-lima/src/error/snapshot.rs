use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("instance '{name}' not found")]
    InstanceNotFound { name: String },
    #[error("snapshot tag '{tag}' not found")]
    TagNotFound { tag: String },
    #[error("snapshot tag '{tag}' already exists")]
    TagAlreadyExists { tag: String },
    #[error("instance '{name}' must be stopped for snapshot operations")]
    MustBeStopped { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for SnapshotError {
    fn from(e: CommandError) -> Self {
        SnapshotError::Command(e)
    }
}
