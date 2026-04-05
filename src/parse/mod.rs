//! Shared parsing logic for git command output.
//! Used by both sync and async implementations.

pub mod branch;
pub mod clone;
pub mod commit;
pub mod diff;
pub mod init;
pub mod log;
pub mod merge;
pub mod open;
pub mod remote;
pub mod stash;
pub mod status;
pub mod tag;
pub mod worktree;
