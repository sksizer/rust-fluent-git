//! Builder for `git stash` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{StashError, CommandError};
use crate::run::{stderr_string, stdout_string};
use crate::types::StashEntry;

/// Entry point builder for stash operations.
pub struct StashBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> StashBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Push changes onto the stash.
    pub fn push(self) -> StashPushBuilder<'a> {
        StashPushBuilder {
            repo_path: self.repo_path,
            message: None,
            include_untracked: false,
        }
    }

    /// Pop the top stash entry.
    pub fn pop(self) -> StashPopBuilder<'a> {
        StashPopBuilder {
            repo_path: self.repo_path,
        }
    }

    /// List all stash entries.
    pub fn list(self) -> StashListBuilder<'a> {
        StashListBuilder {
            repo_path: self.repo_path,
        }
    }
}

// ── Push ───────────────────────────────────────────────────────────────

pub struct StashPushBuilder<'a> {
    repo_path: &'a Path,
    message: Option<String>,
    include_untracked: bool,
}

impl<'a> StashPushBuilder<'a> {
    /// Set a message for the stash entry.
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    /// Include untracked files in the stash.
    pub fn include_untracked(mut self) -> Self {
        self.include_untracked = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("stash")
            .arg("push");

        if let Some(ref msg) = self.message {
            cmd = cmd.arg("-m").arg(msg.as_str());
        }

        if self.include_untracked {
            cmd = cmd.arg("--include-untracked");
        }

        cmd
    }
}

pub(crate) fn parse_push_output(output: &Output) -> Result<(), StashError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        if stdout.contains("No local changes to save") {
            return Err(StashError::NothingToStash);
        }
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("no local changes to save") {
        return Err(StashError::NothingToStash);
    }

    Err(StashError::Command(CommandError::Failed {
        args: "stash push".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Pop ────────────────────────────────────────────────────────────────

pub struct StashPopBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> StashPopBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("stash")
            .arg("pop")
    }
}

pub(crate) fn parse_pop_output(output: &Output) -> Result<(), StashError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("no stash entries found") {
        return Err(StashError::NotFound { index: 0 });
    }

    if lower.contains("conflict") {
        return Err(StashError::ApplyConflict {
            index: 0,
            files: Vec::new(),
        });
    }

    Err(StashError::Command(CommandError::Failed {
        args: "stash pop".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ───────────────────────────────────────────────────────────────

pub struct StashListBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> StashListBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("stash")
            .arg("list")
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<StashEntry>, StashError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        return Err(StashError::Command(CommandError::Failed {
            args: "stash list".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut entries = Vec::new();

    // Format: stash@{0}: On branch_name: message
    // or:     stash@{0}: WIP on branch_name: sha message
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse "stash@{N}: ..."
        let index = if let Some(start) = line.find('{') {
            if let Some(end) = line.find('}') {
                line[start + 1..end].parse::<usize>().unwrap_or(0)
            } else {
                continue;
            }
        } else {
            continue;
        };

        // Everything after "}: "
        let rest = if let Some(pos) = line.find("}: ") {
            &line[pos + 3..]
        } else {
            ""
        };

        // Parse branch and message from "On branch_name: message" or "WIP on branch_name: sha message"
        let (branch, message) = if let Some(stripped) = rest.strip_prefix("On ") {
            if let Some(colon_pos) = stripped.find(": ") {
                (
                    stripped[..colon_pos].to_string(),
                    stripped[colon_pos + 2..].to_string(),
                )
            } else {
                (stripped.to_string(), String::new())
            }
        } else if let Some(stripped) = rest.strip_prefix("WIP on ") {
            if let Some(colon_pos) = stripped.find(": ") {
                (
                    stripped[..colon_pos].to_string(),
                    stripped[colon_pos + 2..].to_string(),
                )
            } else {
                (stripped.to_string(), String::new())
            }
        } else {
            (String::new(), rest.to_string())
        };

        entries.push(StashEntry {
            index,
            message,
            branch,
        });
    }

    Ok(entries)
}
