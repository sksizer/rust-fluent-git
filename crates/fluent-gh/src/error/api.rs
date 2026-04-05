use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not authenticated; run `gh auth login`")]
    NotAuthenticated,
    #[error("HTTP {status}: {message}")]
    HttpError { status: u16, message: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for ApiError {
    fn from(e: CommandError) -> Self {
        ApiError::Command(e)
    }
}
