use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("instance '{name}' not found")]
    NotFound { name: String },
    #[error("instance '{name}' is not running")]
    NotRunning { name: String },
    #[error("command exited with code {code}: {stderr}")]
    NonZeroExit { code: i32, stderr: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for ShellError {
    fn from(e: CommandError) -> Self {
        ShellError::Command(e)
    }
}
