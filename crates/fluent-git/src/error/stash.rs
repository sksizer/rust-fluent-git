use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum StashError {
    #[error("nothing to stash: no local changes to save")]
    NothingToStash,
    #[error("stash index {index} out of range (have {count} stashes)")]
    IndexOutOfRange { index: usize, count: usize },
    #[error("conflict applying stash@{{{index}}}: {files:?}")]
    ApplyConflict { index: usize, files: Vec<String> },
    #[error("stash@{{{index}}} does not exist")]
    NotFound { index: usize },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
