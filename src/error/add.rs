use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum AddError {
    #[error("pathspec '{path}' did not match any files")]
    PathNotFound { path: String },
    #[error("path '{path}' is outside the repository")]
    OutsideRepo { path: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
