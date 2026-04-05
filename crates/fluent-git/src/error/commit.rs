use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    #[error("nothing to commit, working tree clean")]
    NothingToCommit,
    #[error("nothing to commit and --allow-empty not set")]
    EmptyWithoutFlag,
    #[error("commit message is empty")]
    EmptyMessage,
    #[error("committer identity not configured; set user.name and user.email")]
    IdentityNotConfigured,
    #[error("gpg signing failed: {reason}")]
    SigningFailed { reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
