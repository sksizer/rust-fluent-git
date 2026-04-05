mod formula;
mod maintenance;
mod query;
mod services;
mod tap;

pub use formula::FormulaError;
pub use maintenance::MaintenanceError;
pub use query::QueryError;
pub use services::ServiceError;
pub use tap::TapError;

pub use fluent_core::CommandError;

/// Umbrella error for `?` propagation across multiple brew operations.
#[derive(Debug, thiserror::Error)]
pub enum BrewError {
    #[error(transparent)]
    Formula(#[from] FormulaError),
    #[error(transparent)]
    Query(#[from] QueryError),
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    Tap(#[from] TapError),
    #[error(transparent)]
    Maintenance(#[from] MaintenanceError),
}
