use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum ResetError {
    #[error("reset target '{reference}' not found")]
    RefNotFound { reference: String },
    #[error("cannot do hard reset with uncommitted merge")]
    UncommittedMerge,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
