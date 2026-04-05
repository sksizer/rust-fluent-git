use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    #[error("git is not installed or not found in PATH")]
    NotInstalled,
    #[error("git version {version} is below minimum required {minimum}")]
    VersionTooOld { version: String, minimum: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
