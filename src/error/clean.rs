use std::path::PathBuf;

use super::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum CleanError {
    #[error("clean requires --force flag")]
    ForceRequired,
    #[error("refusing to clean: path '{path}' is outside repository")]
    OutsideRepo { path: PathBuf },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
