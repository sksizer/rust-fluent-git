mod copy;
mod disk;
mod info;
mod instance;
mod shell;
mod snapshot;

pub use copy::CopyError;
pub use disk::DiskError;
pub use info::InfoError;
pub use instance::InstanceError;
pub use shell::ShellError;
pub use snapshot::SnapshotError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple lima operations.
#[derive(Debug, thiserror::Error)]
pub enum LimaError {
    #[error(transparent)]
    Instance(#[from] InstanceError),
    #[error(transparent)]
    Shell(#[from] ShellError),
    #[error(transparent)]
    Copy(#[from] CopyError),
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
    #[error(transparent)]
    Disk(#[from] DiskError),
    #[error(transparent)]
    Info(#[from] InfoError),
}
