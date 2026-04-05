//! Builder for `git worktree` operations.

use std::path::{Path, PathBuf};
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::WorktreeError;
use crate::types::{WorktreeInfo, WorktreeListResult, WorktreeLockResult};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ══════════════════════════════════════════════════════════════════════════════
// Entry point
// ══════════════════════════════════════════════════════════════════════════════

/// Entry point builder for worktree operations.
pub struct WorktreeBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> WorktreeBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Add a new worktree at `path` for `branch`.
    pub fn add(self, path: impl Into<PathBuf>, branch: impl Into<String>) -> WorktreeAddBuilder<'a> {
        WorktreeAddBuilder { repo_path: self.repo_path, path: path.into(), branch: branch.into() }
    }

    /// Remove a worktree at `path`.
    pub fn remove(self, path: impl Into<PathBuf>) -> WorktreeRemoveBuilder<'a> {
        WorktreeRemoveBuilder { repo_path: self.repo_path, path: path.into() }
    }

    /// List all worktrees.
    pub fn list(self) -> WorktreeListBuilder<'a> {
        WorktreeListBuilder { repo_path: self.repo_path }
    }

    /// Move a worktree from `old_path` to `new_path`.
    pub fn move_to(self, old_path: impl Into<PathBuf>, new_path: impl Into<PathBuf>) -> WorktreeMoveBuilder<'a> {
        WorktreeMoveBuilder { repo_path: self.repo_path, old_path: old_path.into(), new_path: new_path.into() }
    }

    /// Lock a worktree at `path`.
    pub fn lock(self, path: impl Into<PathBuf>) -> WorktreeLockBuilder<'a> {
        WorktreeLockBuilder { repo_path: self.repo_path, path: path.into(), reason: None }
    }

    /// Unlock a worktree at `path`.
    pub fn unlock(self, path: impl Into<PathBuf>) -> WorktreeUnlockBuilder<'a> {
        WorktreeUnlockBuilder { repo_path: self.repo_path, path: path.into() }
    }

    /// Prune stale worktree metadata.
    pub fn prune(self) -> WorktreePruneBuilder<'a> {
        WorktreePruneBuilder { repo_path: self.repo_path }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Add
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreeAddBuilder<'a> {
    repo_path: &'a Path,
    path: PathBuf,
    branch: String,
}

impl<'a> WorktreeAddBuilder<'a> {
    /// Build the command to check whether the branch already exists.
    pub(crate) fn build_branch_check_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("branch")
            .arg("--list")
            .arg(&self.branch)
    }

    /// Build `git worktree add` with an existing branch.
    pub(crate) fn build_add_existing_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("add")
            .arg(self.path.to_string_lossy().as_ref())
            .arg(&self.branch)
    }

    /// Build `git worktree add -b <branch> <path>` to create a new branch.
    pub(crate) fn build_add_new_branch_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("add")
            .arg("-b")
            .arg(&self.branch)
            .arg(self.path.to_string_lossy().as_ref())
    }

    /// Build `git rev-parse HEAD` to get the SHA after adding.
    pub(crate) fn build_rev_parse_command(&self) -> ShellCommand {
        ShellCommand::new("git").arg("-C").arg(self.path.to_string_lossy().as_ref()).arg("rev-parse").arg("HEAD")
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn branch(&self) -> &str {
        &self.branch
    }
}

pub(crate) fn parse_branch_check_output(output: &Output) -> bool {
    if !output.status.success() {
        return false;
    }
    let stdout = stdout_string(output);
    !stdout.is_empty()
}

pub(crate) fn parse_add_output(output: &Output, path: &Path, branch: &str) -> Result<(), WorktreeError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already checked out") || lower.contains("is already used by worktree") {
        return Err(WorktreeError::BranchInUse { branch: branch.to_string(), path: path.to_path_buf() });
    }

    if lower.contains("already exists") {
        return Err(WorktreeError::AlreadyExists { path: path.to_path_buf() });
    }

    Err(WorktreeError::Command(CommandError::Failed {
        args: format!("worktree add {}", path.display()),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

pub(crate) fn parse_rev_parse_output(output: &Output) -> String {
    stdout_string(output)
}

// ══════════════════════════════════════════════════════════════════════════════
// Remove
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreeRemoveBuilder<'a> {
    repo_path: &'a Path,
    path: PathBuf,
}

impl<'a> WorktreeRemoveBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("remove")
            .arg(self.path.to_string_lossy().as_ref())
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn repo_path(&self) -> &Path {
        self.repo_path
    }
}

pub(crate) fn parse_remove_output(output: &Output, path: &Path) -> Result<(), WorktreeError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not a working tree") || lower.contains("is not a registered worktree") {
        return Err(WorktreeError::NotFound { path: path.to_path_buf() });
    }

    if lower.contains("untracked") || lower.contains("modified") || lower.contains("changes not") {
        return Err(WorktreeError::DirtyWorktree { path: path.to_path_buf() });
    }

    if lower.contains("main working tree") || lower.contains("cannot remove main") {
        return Err(WorktreeError::CannotRemoveMain);
    }

    Err(WorktreeError::Command(CommandError::Failed {
        args: format!("worktree remove {}", path.display()),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ══════════════════════════════════════════════════════════════════════════════
// List
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreeListBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> WorktreeListBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("list")
            .arg("--porcelain")
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<WorktreeListResult, WorktreeError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        return Err(WorktreeError::Command(CommandError::Failed {
            args: "worktree list --porcelain".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut worktrees = Vec::new();

    // Porcelain output: blocks separated by blank lines.
    // Each block has lines like:
    //   worktree /path/to/wt
    //   HEAD abc123
    //   branch refs/heads/main
    //   [locked [reason]]
    // The first block is always the main worktree.
    for block in stdout.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let mut path = PathBuf::new();
        let mut sha = String::new();
        let mut branch = String::new();
        let mut locked = false;

        for line in block.lines() {
            if let Some(rest) = line.strip_prefix("worktree ") {
                path = PathBuf::from(rest);
            } else if let Some(rest) = line.strip_prefix("HEAD ") {
                sha = rest.to_string();
            } else if let Some(rest) = line.strip_prefix("branch ") {
                // Convert refs/heads/main -> main
                branch = rest.strip_prefix("refs/heads/").unwrap_or(rest).to_string();
            } else if line.starts_with("locked") {
                locked = true;
            } else if line == "detached" {
                branch = "(detached)".to_string();
            }
        }

        let is_main = worktrees.is_empty(); // first block is main

        worktrees.push(WorktreeInfo { path, branch, sha, is_main, locked });
    }

    let main = worktrees.first().cloned().unwrap_or_else(|| WorktreeInfo {
        path: PathBuf::new(),
        branch: String::new(),
        sha: String::new(),
        is_main: true,
        locked: false,
    });

    Ok(WorktreeListResult { worktrees, main })
}

// ══════════════════════════════════════════════════════════════════════════════
// Move
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreeMoveBuilder<'a> {
    repo_path: &'a Path,
    old_path: PathBuf,
    new_path: PathBuf,
}

impl<'a> WorktreeMoveBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("move")
            .arg(self.old_path.to_string_lossy().as_ref())
            .arg(self.new_path.to_string_lossy().as_ref())
    }

    pub(crate) fn old_path(&self) -> &Path {
        &self.old_path
    }

    pub(crate) fn new_path(&self) -> &Path {
        &self.new_path
    }

    pub(crate) fn repo_path(&self) -> &Path {
        self.repo_path
    }
}

pub(crate) fn parse_move_output(output: &Output, old_path: &Path) -> Result<(), WorktreeError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not a working tree") || lower.contains("is not a registered worktree") {
        return Err(WorktreeError::NotFound { path: old_path.to_path_buf() });
    }

    if lower.contains("main working tree") || lower.contains("cannot move main") {
        return Err(WorktreeError::CannotRemoveMain);
    }

    Err(WorktreeError::Command(CommandError::Failed {
        args: format!("worktree move {}", old_path.display()),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ══════════════════════════════════════════════════════════════════════════════
// Lock
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreeLockBuilder<'a> {
    repo_path: &'a Path,
    path: PathBuf,
    reason: Option<String>,
}

impl<'a> WorktreeLockBuilder<'a> {
    /// Provide an optional reason for locking.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("lock");

        if let Some(ref reason) = self.reason {
            cmd = cmd.arg("--reason").arg(reason.as_str());
        }

        cmd = cmd.arg(self.path.to_string_lossy().as_ref());
        cmd
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn reason_ref(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

pub(crate) fn parse_lock_output(
    output: &Output,
    path: &Path,
    reason: Option<&str>,
) -> Result<WorktreeLockResult, WorktreeError> {
    if output.status.success() {
        return Ok(WorktreeLockResult {
            path: path.to_path_buf(),
            reason: reason.map(|s| s.to_string()),
            was_already_locked: false,
        });
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already locked") || lower.contains("is locked") {
        return Ok(WorktreeLockResult {
            path: path.to_path_buf(),
            reason: reason.map(|s| s.to_string()),
            was_already_locked: true,
        });
    }

    if lower.contains("not a working tree") || lower.contains("is not a registered worktree") {
        return Err(WorktreeError::NotFound { path: path.to_path_buf() });
    }

    Err(WorktreeError::Command(CommandError::Failed {
        args: format!("worktree lock {}", path.display()),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ══════════════════════════════════════════════════════════════════════════════
// Unlock
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreeUnlockBuilder<'a> {
    repo_path: &'a Path,
    path: PathBuf,
}

impl<'a> WorktreeUnlockBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("unlock")
            .arg(self.path.to_string_lossy().as_ref())
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

pub(crate) fn parse_unlock_output(output: &Output, path: &Path) -> Result<(), WorktreeError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not locked") {
        // Unlocking an already-unlocked worktree is not an error.
        return Ok(());
    }

    if lower.contains("not a working tree") || lower.contains("is not a registered worktree") {
        return Err(WorktreeError::NotFound { path: path.to_path_buf() });
    }

    Err(WorktreeError::Command(CommandError::Failed {
        args: format!("worktree unlock {}", path.display()),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ══════════════════════════════════════════════════════════════════════════════
// Prune
// ══════════════════════════════════════════════════════════════════════════════

pub struct WorktreePruneBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> WorktreePruneBuilder<'a> {
    /// Build the dry-run command to discover what would be pruned.
    pub(crate) fn build_dry_run_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("worktree")
            .arg("prune")
            .arg("--dry-run")
    }

    /// Build the actual prune command.
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("worktree").arg("prune")
    }
}

pub(crate) fn parse_prune_dry_run_output(output: &Output) -> Result<Vec<PathBuf>, WorktreeError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        return Err(WorktreeError::Command(CommandError::Failed {
            args: "worktree prune --dry-run".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut pruned = Vec::new();

    // Output lines like: "Removing worktrees/foo: gitdir file points to non-existent location"
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Extract the path from "Removing <path>: ..."
        if let Some(rest) = line.strip_prefix("Removing ") {
            if let Some(colon_pos) = rest.find(':') {
                pruned.push(PathBuf::from(rest[..colon_pos].trim()));
            } else {
                pruned.push(PathBuf::from(rest.trim()));
            }
        }
    }

    Ok(pruned)
}

pub(crate) fn parse_prune_output(output: &Output) -> Result<(), WorktreeError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(WorktreeError::Command(CommandError::Failed {
        args: "worktree prune".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Helper to find branch for a worktree path (used by remove/move) ────────

pub(crate) fn build_list_command(repo_path: &Path) -> ShellCommand {
    ShellCommand::new("git")
        .arg("-C")
        .arg(repo_path.to_string_lossy().as_ref())
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
}

pub(crate) fn find_branch_for_path(list_output: &Output, path: &Path) -> String {
    let stdout = stdout_string(list_output);
    let mut current_path = PathBuf::new();
    let mut current_branch = String::new();

    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("worktree ") {
            current_path = PathBuf::from(rest);
            current_branch.clear();
        } else if let Some(rest) = line.strip_prefix("branch ") {
            current_branch = rest.strip_prefix("refs/heads/").unwrap_or(rest).to_string();
        } else if line.is_empty() {
            if current_path == path {
                return current_branch;
            }
            current_branch.clear();
        }
    }

    // Check last block (no trailing blank line)
    if current_path == path {
        return current_branch;
    }

    String::new()
}
