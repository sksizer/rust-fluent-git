mod issue;
mod pr;
mod release;
mod repo;
mod run;
mod workflow;

pub use issue::IssueError;
pub use pr::PrError;
pub use release::ReleaseError;
pub use repo::RepoError;
pub use run::RunError;
pub use workflow::WorkflowError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple gh operations.
#[derive(Debug, thiserror::Error)]
pub enum GhError {
    #[error(transparent)]
    Pr(#[from] PrError),
    #[error(transparent)]
    Issue(#[from] IssueError),
    #[error(transparent)]
    Release(#[from] ReleaseError),
    #[error(transparent)]
    Repo(#[from] RepoError),
    #[error(transparent)]
    Run(#[from] RunError),
    #[error(transparent)]
    Workflow(#[from] WorkflowError),
}
