use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum TapError {
    #[error("'{name}' is already tapped")]
    AlreadyTapped { name: String },
    #[error("'{name}' is not tapped")]
    NotTapped { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for TapError {
    fn from(e: CommandError) -> Self {
        TapError::Command(e)
    }
}
