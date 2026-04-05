use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum RevParseError {
    #[error("ref '{reference}' not found")]
    RefNotFound { reference: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
