use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum FormulaError {
    #[error("formula or cask '{name}' not found")]
    NotFound { name: String },
    #[error("'{name}' is already installed")]
    AlreadyInstalled { name: String },
    #[error("'{name}' is not installed")]
    NotInstalled { name: String },
    #[error("'{name}' is not pinned")]
    NotPinned { name: String },
    #[error("'{name}' is already linked")]
    AlreadyLinked { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for FormulaError {
    fn from(e: CommandError) -> Self {
        FormulaError::Command(e)
    }
}
