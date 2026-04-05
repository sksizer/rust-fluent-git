//! Shared parsing logic for git command output.
//! Used by both sync and async implementations.

pub mod init;
pub mod open;
pub mod clone;
pub mod status;
pub mod log;
pub mod diff;
pub mod commit;
pub mod branch;
pub mod remote;
pub mod tag;
pub mod stash;
pub mod merge;
pub mod worktree;
