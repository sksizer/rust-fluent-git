use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum StatusError {
    #[error("index is locked by another process")]
    IndexLocked,
    #[error("index is corrupt: {reason}")]
    CorruptIndex { reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
