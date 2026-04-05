pub mod error;
pub mod run;
pub mod tool;

pub use error::CommandError;
pub use run::{stderr_string, stdout_string};

#[cfg(feature = "blocking")]
pub use run::run_sync;

#[cfg(feature = "tokio")]
pub use run::run_async;

// Re-export cmd-spec for tool crates
pub use cmd_spec::ShellCommand;
