#[derive(Debug, thiserror::Error)]
pub enum R2Error {
    #[error("bucket not found")]
    BucketNotFound,
    #[error("bucket already exists")]
    BucketAlreadyExists,
    #[error("object not found")]
    ObjectNotFound,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
