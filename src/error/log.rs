use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("ref '{reference}' not found")]
    RefNotFound { reference: String },
    #[error("no commits yet")]
    NoCommits,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
