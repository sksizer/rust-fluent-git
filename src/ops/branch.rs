//! Builder for `git branch` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{BranchError, CommandError};
use crate::run::{stderr_string, stdout_string};
use crate::types::BranchInfo;

/// Entry point builder for branch operations.
pub struct BranchBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> BranchBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Create a new branch.
    pub fn create(self, name: impl Into<String>) -> BranchCreateBuilder<'a> {
        BranchCreateBuilder {
            repo_path: self.repo_path,
            name: name.into(),
        }
    }

    /// Delete a branch.
    pub fn delete(self, name: impl Into<String>) -> BranchDeleteBuilder<'a> {
        BranchDeleteBuilder {
            repo_path: self.repo_path,
            name: name.into(),
        }
    }

    /// List branches.
    pub fn list(self) -> BranchListBuilder<'a> {
        BranchListBuilder {
            repo_path: self.repo_path,
        }
    }

    /// Rename a branch.
    pub fn rename(self, old: impl Into<String>, new: impl Into<String>) -> BranchRenameBuilder<'a> {
        BranchRenameBuilder {
            repo_path: self.repo_path,
            old_name: old.into(),
            new_name: new.into(),
        }
    }

    /// Access the repo path for sync/async current() impls.
    pub(crate) fn repo_path(&self) -> &Path {
        self.repo_path
    }
}

pub(crate) fn build_current_command(repo_path: &Path) -> ShellCommand {
    ShellCommand::new("git")
        .arg("-C")
        .arg(repo_path.to_string_lossy().as_ref())
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
}

pub(crate) fn parse_current_output(output: &Output) -> Result<String, BranchError> {
    if output.status.success() {
        Ok(stdout_string(output))
    } else {
        let stderr = stderr_string(output);
        Err(BranchError::Command(CommandError::Failed {
            args: "symbolic-ref --short HEAD".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }))
    }
}

// ── Create ──────────────────────────────────────────────────────────────

pub struct BranchCreateBuilder<'a> {
    repo_path: &'a Path,
    name: String,
}

impl<'a> BranchCreateBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("branch")
            .arg(&self.name)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_create_output(output: &Output, name: &str) -> Result<(), BranchError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(BranchError::AlreadyExists {
            name: name.to_string(),
        });
    }

    if lower.contains("not a valid branch name") || lower.contains("invalid branch name") {
        return Err(BranchError::InvalidName {
            name: name.to_string(),
            reason: stderr.clone(),
        });
    }

    Err(BranchError::Command(CommandError::Failed {
        args: format!("branch {name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ───────────────────���──────────────────────────────────────────

pub struct BranchDeleteBuilder<'a> {
    repo_path: &'a Path,
    name: String,
}

impl<'a> BranchDeleteBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("branch")
            .arg("-d")
            .arg(&self.name)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str) -> Result<(), BranchError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("checked out") || lower.contains("used by worktree") || lower.contains("cannot delete branch") && lower.contains("checked out") {
        return Err(BranchError::DeleteCurrent {
            name: name.to_string(),
        });
    }

    if lower.contains("not fully merged") {
        return Err(BranchError::NotFullyMerged {
            name: name.to_string(),
        });
    }

    if lower.contains("not found") {
        return Err(BranchError::NotFound {
            name: name.to_string(),
        });
    }

    Err(BranchError::Command(CommandError::Failed {
        args: format!("branch -d {name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ───────────────��────────────────────────────���───────────────────

pub struct BranchListBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> BranchListBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("branch")
            .arg("-vv")
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<BranchInfo>, BranchError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        return Err(BranchError::Command(CommandError::Failed {
            args: "branch -vv".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut branches = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let is_current = line.starts_with('*');
        let line = line.trim_start_matches('*').trim();

        // Format: "name    sha commit message [upstream] ..."
        // or:     "name    sha commit message"
        let mut parts = line.splitn(3, char::is_whitespace);
        let name = match parts.next() {
            Some(n) => n.to_string(),
            None => continue,
        };

        // Skip whitespace and get sha
        let sha = parts
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        let rest = parts.next().unwrap_or("").trim();

        // Check for upstream tracking info: [origin/main] or [origin/main: ahead 1]
        let (upstream, commit_msg) = if rest.starts_with('[') {
            if let Some(bracket_end) = rest.find(']') {
                let upstream_str = &rest[1..bracket_end];
                // upstream may contain ": ahead N" or ": behind N"
                let upstream_name = upstream_str.split(':').next().unwrap_or("").trim();
                let msg = rest[bracket_end + 1..].trim();
                (Some(upstream_name.to_string()), msg.to_string())
            } else {
                (None, rest.to_string())
            }
        } else {
            (None, rest.to_string())
        };

        branches.push(BranchInfo {
            name,
            is_current,
            upstream,
            sha,
            last_commit_message: commit_msg,
        });
    }

    Ok(branches)
}

// ── Rename ─────────────��───────────────────────────���────────────────────

pub struct BranchRenameBuilder<'a> {
    repo_path: &'a Path,
    old_name: String,
    new_name: String,
}

impl<'a> BranchRenameBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("branch")
            .arg("-m")
            .arg(&self.old_name)
            .arg(&self.new_name)
    }

    pub(crate) fn old_name(&self) -> &str {
        &self.old_name
    }
}

pub(crate) fn parse_rename_output(output: &Output, old_name: &str) -> Result<(), BranchError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("did not match") {
        return Err(BranchError::NotFound {
            name: old_name.to_string(),
        });
    }

    Err(BranchError::Command(CommandError::Failed {
        args: format!("branch -m {old_name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
