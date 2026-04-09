#[derive(Debug, thiserror::Error)]
pub enum D1Error {
    #[error("database not found")]
    NotFound,
    #[error("database already exists")]
    AlreadyExists,
    #[error("SQL error")]
    SqlError,
    #[error("migration failed")]
    MigrationFailed,
    #[error("command failed: {0}")]
    Command(#[from] fluent_core::CommandError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
