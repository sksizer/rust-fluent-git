use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("not authenticated")]
    NotAuthenticated,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for AuthError {
    fn from(e: CommandError) -> Self {
        AuthError::Command(e)
    }
}
