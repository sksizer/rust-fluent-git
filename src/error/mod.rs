mod add;
mod branch;
mod checkout;
mod cherry_pick;
mod clean;
mod clone;
mod commit;
mod config;
mod diff;
mod init;
mod log;
mod merge;
mod open;
mod rebase;
mod remote;
mod reset;
mod rev_parse;
mod setup;
mod stash;
mod status;
mod tag;
mod worktree;

pub use add::AddError;
pub use branch::BranchError;
pub use checkout::CheckoutError;
pub use cherry_pick::CherryPickError;
pub use clean::CleanError;
pub use clone::CloneError;
pub use commit::CommitError;
pub use config::ConfigError;
pub use diff::DiffError;
pub use init::InitError;
pub use log::LogError;
pub use merge::MergeError;
pub use open::OpenError;
pub use rebase::RebaseError;
pub use remote::RemoteError;
pub use reset::ResetError;
pub use rev_parse::RevParseError;
pub use setup::SetupError;
pub use stash::StashError;
pub use status::StatusError;
pub use tag::TagError;
pub use worktree::WorktreeError;

// ══════════════════════════════════════════════════════════════════════════════
//
// SHARED COMMAND ERROR
//
// Not a top-level domain error. Embedded in each domain error as a fallback
// for "git exited non-zero in a way we didn't categorize."
//
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("command `git {args}` failed (exit code {code}):\n{stderr}")]
    Failed { args: String, code: i32, stdout: String, stderr: String },

    #[error("command `git {args}` timed out after {timeout_secs}s")]
    Timeout { args: String, timeout_secs: u64 },

    #[error("command `git {args}` was interrupted by signal {signal}")]
    Signal { args: String, signal: i32 },
}

// ══════════════════════════════════════════════════════════════════════════════
//
// GitError — umbrella for ? propagation across operations
//
// You never see this in an individual .run() return type. It only appears
// when a function combines multiple operations:
//
//   fn workflow(repo: &Repo) -> Result<(), GitError> {
//       repo.add().all().run()?;             // AddError -> GitError
//       repo.commit().message("x").run()?;   // CommitError -> GitError
//       Ok(())
//   }
//
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    // Setup
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Init(#[from] InitError),
    #[error(transparent)]
    Open(#[from] OpenError),
    #[error(transparent)]
    Clone(#[from] CloneError),
    // Repo operations
    #[error(transparent)]
    Add(#[from] AddError),
    #[error(transparent)]
    Branch(#[from] BranchError),
    #[error(transparent)]
    Checkout(#[from] CheckoutError),
    #[error(transparent)]
    Commit(#[from] CommitError),
    #[error(transparent)]
    Status(#[from] StatusError),
    #[error(transparent)]
    Log(#[from] LogError),
    #[error(transparent)]
    Merge(#[from] MergeError),
    #[error(transparent)]
    Rebase(#[from] RebaseError),
    #[error(transparent)]
    CherryPick(#[from] CherryPickError),
    #[error(transparent)]
    Remote(#[from] RemoteError),
    #[error(transparent)]
    Stash(#[from] StashError),
    #[error(transparent)]
    Tag(#[from] TagError),
    #[error(transparent)]
    Worktree(#[from] WorktreeError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Clean(#[from] CleanError),
    #[error(transparent)]
    Reset(#[from] ResetError),
    #[error(transparent)]
    Diff(#[from] DiffError),
    #[error(transparent)]
    RevParse(#[from] RevParseError),
}
