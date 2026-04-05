//! Builder for `git rebase` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, RebaseError};
use crate::run::{stderr_string, stdout_string};

/// Builder for a `git rebase` command.
pub struct RebaseBuilder<'a> {
    repo_path: &'a Path,
    onto: Option<String>,
}

impl<'a> RebaseBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self {
            repo_path,
            onto: None,
        }
    }

    /// Set the ref to rebase onto.
    pub fn onto(mut self, reference: impl Into<String>) -> Self {
        self.onto = Some(reference.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("rebase");

        if let Some(ref onto) = self.onto {
            cmd = cmd.arg(onto.as_str());
        }

        cmd
    }

    pub(crate) fn onto_ref(&self) -> &Option<String> {
        &self.onto
    }
}

/// Parse rebase output for errors.
pub(crate) fn parse_rebase_output(output: &Output, onto: &Option<String>) -> Result<(), RebaseError> {
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
        return Err(RebaseError::Conflict { files });
    }

    // Check for dirty work tree
    if combined.contains("uncommitted changes") || combined.contains("unstaged changes") {
        let files = parse_dirty_files(&combined);
        return Err(RebaseError::DirtyWorkTree { files });
    }

    // Check for ref not found
    if stderr.contains("unknown revision")
        || stderr.contains("does not point to a commit")
        || stderr.contains("invalid upstream")
    {
        let reference = onto.clone().unwrap_or_default();
        return Err(RebaseError::RefNotFound { reference });
    }

    Err(RebaseError::Command(CommandError::Failed {
        args: "rebase".to_string(),
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
