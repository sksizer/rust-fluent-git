mod issue;
mod pr;
mod repo;

pub use issue::IssueError;
pub use pr::PrError;
pub use repo::RepoError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple gh operations.
#[derive(Debug, thiserror::Error)]
pub enum GhError {
    #[error(transparent)]
    Pr(#[from] PrError),
    #[error(transparent)]
    Issue(#[from] IssueError),
    #[error(transparent)]
    Repo(#[from] RepoError),
}
