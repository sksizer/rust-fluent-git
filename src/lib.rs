pub mod error;
pub mod types;
pub mod run;
pub mod parse;
pub mod ops;

#[cfg(feature = "blocking")]
pub mod sync;

#[cfg(feature = "tokio")]
pub mod git;

// Legacy modules — will be removed once migration is complete
pub mod builder;
pub mod cmd;
pub mod info;

pub use cmd::GitCommand;
pub use cmd::git as git_cmd;
pub use error::GitError;
