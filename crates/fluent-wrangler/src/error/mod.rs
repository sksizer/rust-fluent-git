mod account;
mod d1;
mod deployment;
mod hyperdrive;
mod kv;
mod pages;
mod queues;
mod r2;
mod secret;
mod vectorize;
mod version;
mod worker;
mod workflows;

pub use account::AccountError;
pub use d1::D1Error;
pub use deployment::DeploymentError;
pub use hyperdrive::HyperdriveError;
pub use kv::KvError;
pub use pages::PagesError;
pub use queues::QueuesError;
pub use r2::R2Error;
pub use secret::SecretError;
pub use vectorize::VectorizeError;
pub use version::VersionError;
pub use worker::WorkerError;
pub use workflows::WorkflowsError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple wrangler operations.
#[derive(Debug, thiserror::Error)]
pub enum WranglerError {
    #[error(transparent)]
    Account(#[from] AccountError),
    #[error(transparent)]
    Worker(#[from] WorkerError),
    #[error(transparent)]
    Deployment(#[from] DeploymentError),
    #[error(transparent)]
    Version(#[from] VersionError),
    #[error(transparent)]
    Secret(#[from] SecretError),
    #[error(transparent)]
    D1(#[from] D1Error),
    #[error(transparent)]
    Kv(#[from] KvError),
    #[error(transparent)]
    R2(#[from] R2Error),
    #[error(transparent)]
    Pages(#[from] PagesError),
    #[error(transparent)]
    Queues(#[from] QueuesError),
    #[error(transparent)]
    Vectorize(#[from] VectorizeError),
    #[error(transparent)]
    Hyperdrive(#[from] HyperdriveError),
    #[error(transparent)]
    Workflows(#[from] WorkflowsError),
}
