use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum DiffError {
    #[error("diff ref '{reference}' not found")]
    RefNotFound { reference: String },
    #[error("bad revision range '{range}'")]
    InvalidRange { range: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
