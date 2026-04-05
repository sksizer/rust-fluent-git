mod repo;
mod add;
mod branch;
mod checkout;
mod clean;
mod clone;
mod commit;
mod config;
mod diff;
mod init;
mod log;
mod merge;
mod remote;
mod reset;
mod rev_parse;
mod setup;
mod stash;
mod status;
mod tag;
mod worktree;

pub use repo::Repo;
pub use add::AddResult;
pub use branch::BranchInfo;
pub use clean::CleanResult;
pub use clone::CloneResult;
pub use commit::CommitResult;
pub use diff::{DiffFile, DiffResult, DiffStats};
pub use init::InitResult;
pub use log::LogEntry;
pub use merge::MergeResult;
pub use remote::RemoteInfo;
pub use reset::{ResetMode, ResetResult};
pub use setup::GitInfo;
pub use stash::StashEntry;
pub use status::StatusResult;
pub use tag::TagInfo;
pub use worktree::{
    WorktreeAddResult, WorktreeInfo, WorktreeListResult, WorktreeLockResult, WorktreeMoveResult,
    WorktreePruneResult, WorktreeRemoveResult,
};

// Shared types used across multiple domains
#[derive(Debug, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Untracked,
}

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub status: FileStatus,
    pub old_path: Option<String>,
}
