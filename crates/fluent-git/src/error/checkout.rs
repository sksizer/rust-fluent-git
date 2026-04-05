use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum CheckoutError {
    #[error("pathspec '{reference}' did not match any known ref")]
    RefNotFound { reference: String },
    #[error("cannot checkout: uncommitted changes would be overwritten in: {files:?}")]
    UncommittedChanges { files: Vec<String> },
    #[error("cannot create branch '{name}': already exists")]
    BranchAlreadyExists { name: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
