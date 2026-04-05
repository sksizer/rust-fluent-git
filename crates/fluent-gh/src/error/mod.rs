mod api;
mod auth;
mod issue;
mod label;
mod pr;
mod release;
mod repo;
mod run;
mod secret;
mod variable;
mod workflow;

pub use api::ApiError;
pub use auth::AuthError;
pub use issue::IssueError;
pub use label::LabelError;
pub use pr::PrError;
pub use release::ReleaseError;
pub use repo::RepoError;
pub use run::RunError;
pub use secret::SecretError;
pub use variable::VariableError;
pub use workflow::WorkflowError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple gh operations.
#[derive(Debug, thiserror::Error)]
pub enum GhError {
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Pr(#[from] PrError),
    #[error(transparent)]
    Issue(#[from] IssueError),
    #[error(transparent)]
    Label(#[from] LabelError),
    #[error(transparent)]
    Release(#[from] ReleaseError),
    #[error(transparent)]
    Repo(#[from] RepoError),
    #[error(transparent)]
    Run(#[from] RunError),
    #[error(transparent)]
    Secret(#[from] SecretError),
    #[error(transparent)]
    Variable(#[from] VariableError),
    #[error(transparent)]
    Workflow(#[from] WorkflowError),
}
