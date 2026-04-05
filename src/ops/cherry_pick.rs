//! Builder for `git cherry-pick` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CherryPickError, CommandError};
use crate::run::{stderr_string, stdout_string};

/// Builder for a `git cherry-pick` command.
pub struct CherryPickBuilder<'a> {
    repo_path: &'a Path,
    sha: Option<String>,
}

impl<'a> CherryPickBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self {
            repo_path,
            sha: None,
        }
    }

    /// Set the commit SHA to cherry-pick.
    pub fn commit(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("cherry-pick");

        if let Some(ref sha) = self.sha {
            cmd = cmd.arg(sha.as_str());
        }

        cmd
    }

    pub(crate) fn sha_ref(&self) -> &Option<String> {
        &self.sha
    }
}

/// Parse cherry-pick output for errors.
pub(crate) fn parse_cherry_pick_output(output: &Output, sha: &Option<String>) -> Result<(), CherryPickError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let stdout = stdout_string(output);
    let combined = format!("{stderr} {stdout}");
    let code = output.status.code().unwrap_or(-1);

    // Check for conflict
    if combined.contains("CONFLICT") || combined.contains("could not apply") {
        let files = parse_conflict_files(&combined);
        return Err(CherryPickError::Conflict { files });
    }

    // Check for commit not found
    if stderr.contains("bad object") || stderr.contains("unknown revision") {
        let sha = sha.clone().unwrap_or_default();
        return Err(CherryPickError::CommitNotFound { sha });
    }

    // Check for dirty work tree
    if combined.contains("uncommitted changes") || combined.contains("local changes") {
        let files = parse_dirty_files(&combined);
        return Err(CherryPickError::DirtyWorkTree { files });
    }

    Err(CherryPickError::Command(CommandError::Failed {
        args: "cherry-pick".to_string(),
        code,
        stdout,
        stderr,
    }))
}

fn parse_conflict_files(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|l| l.contains("CONFLICT"))
        .filter_map(|l| {
            l.rsplit("Merge conflict in ").next().map(|s| s.trim().to_string())
        })
        .filter(|s| !s.is_empty() && !s.contains("CONFLICT"))
        .collect()
}

fn parse_dirty_files(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|l| l.starts_with('\t'))
        .map(|l| l.trim().to_string())
        .collect()
}
