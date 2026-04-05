//! Builder for `git merge` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, MergeError};
use crate::run::{stderr_string, stdout_string};
use crate::types::MergeResult;

/// Builder for a `git merge` command.
pub struct MergeBuilder<'a> {
    repo_path: &'a Path,
    branch: Option<String>,
    no_ff: bool,
    squash: bool,
}

impl<'a> MergeBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path, branch: None, no_ff: false, squash: false }
    }

    /// Set the branch to merge.
    pub fn branch(mut self, name: impl Into<String>) -> Self {
        self.branch = Some(name.into());
        self
    }

    /// Disable fast-forward merges.
    pub fn no_ff(mut self) -> Self {
        self.no_ff = true;
        self
    }

    /// Squash the merge into a single commit.
    pub fn squash(mut self) -> Self {
        self.squash = true;
        self
    }

    pub(crate) fn build_merge_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("merge");

        if self.no_ff {
            cmd = cmd.arg("--no-ff");
        }

        if self.squash {
            cmd = cmd.arg("--squash");
        }

        if let Some(ref branch) = self.branch {
            cmd = cmd.arg(branch.as_str());
        }

        cmd
    }

    pub(crate) fn build_log_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("log")
            .arg("-1")
            .arg("--format=%H%n%s")
            .arg("--shortstat")
    }

    pub(crate) fn branch_ref(&self) -> &Option<String> {
        &self.branch
    }
}

/// Parse merge command output for errors.
pub(crate) fn parse_merge_output(output: &Output, branch: &Option<String>) -> Result<(), MergeError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let stdout = stdout_string(output);
    let combined = format!("{stderr} {stdout}");
    let code = output.status.code().unwrap_or(-1);

    // Check for conflict
    if combined.contains("CONFLICT") || combined.contains("Automatic merge failed") {
        let files = parse_conflict_files(&combined);
        return Err(MergeError::Conflict { files });
    }

    // Check for dirty work tree
    if combined.contains("uncommitted changes") || combined.contains("local changes") {
        let files = parse_dirty_files(&combined);
        return Err(MergeError::DirtyWorkTree { files });
    }

    // Check for ref not found
    if stderr.contains("not something we can merge")
        || stderr.contains("unknown revision")
        || stderr.contains("does not point to a commit")
    {
        let reference = branch.clone().unwrap_or_default();
        return Err(MergeError::RefNotFound { reference });
    }

    Err(MergeError::Command(CommandError::Failed { args: "merge".to_string(), code, stdout, stderr }))
}

/// Parse `git log -1` output after a successful merge.
pub(crate) fn parse_merge_details(output: &Output, merge_stdout: &str) -> Result<MergeResult, MergeError> {
    let stdout = stdout_string(output);
    let lines: Vec<&str> = stdout.lines().collect();

    let sha = lines.first().map(|s| s.to_string()).unwrap_or_default();
    let subject = lines.get(1).copied().unwrap_or("");

    // Determine if it was a fast-forward from the merge output
    let fast_forward = merge_stdout.contains("Fast-forward") || merge_stdout.contains("fast-forward");

    // Extract strategy from merge output
    let strategy = if fast_forward {
        "fast-forward".to_string()
    } else if merge_stdout.contains("recursive") {
        "recursive".to_string()
    } else if merge_stdout.contains("ort") {
        "ort".to_string()
    } else if subject.starts_with("Merge") {
        "merge".to_string()
    } else {
        "unknown".to_string()
    };

    // Parse shortstat for files_changed
    let files_changed = lines
        .iter()
        .skip(2)
        .find(|l| l.contains("changed") || l.contains("insertion") || l.contains("deletion"))
        .map(|l| parse_files_changed(l))
        .unwrap_or(0);

    Ok(MergeResult { sha, strategy, fast_forward, files_changed })
}

fn parse_files_changed(line: &str) -> usize {
    for part in line.split(',') {
        let part = part.trim();
        if part.contains("file")
            && let Some(num_str) = part.split_whitespace().next()
            && let Ok(num) = num_str.parse::<usize>()
        {
            return num;
        }
    }
    0
}

fn parse_conflict_files(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|l| l.contains("CONFLICT"))
        .filter_map(|l| {
            // "CONFLICT (content): Merge conflict in <file>"
            l.rsplit("Merge conflict in ").next().map(|s| s.trim().to_string())
        })
        .filter(|s| !s.is_empty() && !s.contains("CONFLICT"))
        .collect()
}

fn parse_dirty_files(output: &str) -> Vec<String> {
    output.lines().filter(|l| l.starts_with('\t')).map(|l| l.trim().to_string()).collect()
}
