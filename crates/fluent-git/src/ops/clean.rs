//! Builder for `git clean` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::CleanError;
use crate::types::CleanResult;
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

/// Builder for a `git clean` command.
pub struct CleanBuilder<'a> {
    repo_path: &'a Path,
    force: bool,
    directories: bool,
}

impl<'a> CleanBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path, force: false, directories: false }
    }

    /// Enable force mode (required by git clean).
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    /// Also remove untracked directories.
    pub fn directories(mut self) -> Self {
        self.directories = true;
        self
    }

    /// Check if force is set (used before running).
    pub(crate) fn is_force(&self) -> bool {
        self.force
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("clean");

        if self.force {
            cmd = cmd.arg("-f");
        }

        if self.directories {
            cmd = cmd.arg("-d");
        }

        cmd
    }
}

/// Parse clean command output for errors.
pub(crate) fn parse_clean_output(output: &Output) -> Result<CleanResult, CleanError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        return Ok(parse_clean_lines(&stdout));
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    if stderr.contains("clean.requireForce") || stderr.contains("refusing to clean") {
        return Err(CleanError::ForceRequired);
    }

    Err(CleanError::Command(CommandError::Failed {
        args: "clean".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

/// Parse lines like "Removing file.txt" or "Removing dir/"
fn parse_clean_lines(stdout: &str) -> CleanResult {
    let mut removed_files = Vec::new();
    let mut removed_dirs = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if let Some(path) = line.strip_prefix("Removing ") {
            let path = path.trim();
            if path.ends_with('/') {
                removed_dirs.push(path.trim_end_matches('/').to_string());
            } else {
                removed_files.push(path.to_string());
            }
        }
    }

    CleanResult { removed_files, removed_dirs }
}
