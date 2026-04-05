pub mod error;
pub mod ops;
pub mod types;

#[cfg(feature = "blocking")]
pub mod sync;

#[cfg(feature = "tokio")]
pub mod claude;

pub use error::ClaudeError;
pub use types::ClaudeCode;

// Re-export core for convenience
pub use fluent_core as core;
