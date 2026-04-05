//! Builder for `git status` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, StatusError};
use crate::run::{stderr_string, stdout_string};
use crate::types::{FileChange, FileStatus, StatusResult};

/// Builder for a `git status` command.
pub struct StatusBuilder<'a> {
    repo_path: &'a Path,
    short: bool,
}

impl<'a> StatusBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path, short: false }
    }

    /// Show short-format output.
    pub fn short(mut self) -> Self {
        self.short = true;
        self
    }

    /// Scoped mutation via closure.
    pub fn with(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("status")
            .arg("--porcelain=v1")
            .arg("-b");

        if self.short {
            cmd = cmd.arg("--short");
        }

        cmd
    }
}

pub(crate) fn parse_status_output(output: &Output) -> Result<StatusResult, StatusError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        let code = output.status.code().unwrap_or(-1);

        if stderr.contains("index.lock") {
            return Err(StatusError::IndexLocked);
        }

        if stderr.contains("index file corrupt") {
            return Err(StatusError::CorruptIndex { reason: stderr.clone() });
        }

        return Err(StatusError::Command(CommandError::Failed {
            args: "status".to_string(),
            code,
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut branch = String::new();
    let mut staged = Vec::new();
    let mut modified = Vec::new();
    let mut untracked = Vec::new();
    let mut ahead: usize = 0;
    let mut behind: usize = 0;

    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            // Parse branch header: "branch...tracking [ahead N, behind M]"
            // or just "branch" or "No commits yet on main"
            let branch_part: &str = if let Some(bracket_start) = rest.find('[') {
                let bracket_end = rest.find(']').unwrap_or(rest.len());
                let info = &rest[bracket_start + 1..bracket_end];
                for part in info.split(',') {
                    let part = part.trim();
                    if let Some(n) = part.strip_prefix("ahead ") {
                        ahead = n.trim().parse().unwrap_or(0);
                    } else if let Some(n) = part.strip_prefix("behind ") {
                        behind = n.trim().parse().unwrap_or(0);
                    }
                }
                rest[..bracket_start].trim_end()
            } else {
                rest
            };

            // branch_part is "branch...tracking" or just "branch"
            branch = if let Some(dots) = branch_part.find("...") {
                branch_part[..dots].to_string()
            } else if let Some(name) = branch_part.strip_prefix("No commits yet on ") {
                name.to_string()
            } else {
                branch_part.to_string()
            };
        } else if line.len() >= 3 {
            let x = line.as_bytes()[0];
            let y = line.as_bytes()[1];
            // line[2] should be a space
            let path = line[3..].to_string();

            // Handle renames: "R  old -> new"
            let (file_path, old_path) = if path.contains(" -> ") {
                let parts: Vec<&str> = path.splitn(2, " -> ").collect();
                (parts[1].to_string(), Some(parts[0].to_string()))
            } else {
                (path, None)
            };

            if x == b'?' && y == b'?' {
                untracked.push(file_path);
            } else {
                // Index (staged) status
                if x != b' ' && x != b'?' {
                    let status = char_to_file_status(x);
                    staged.push(FileChange { path: file_path.clone(), status, old_path: old_path.clone() });
                }

                // Worktree (modified) status
                if y != b' ' && y != b'?' {
                    let status = char_to_file_status(y);
                    modified.push(FileChange { path: file_path, status, old_path });
                }
            }
        }
    }

    Ok(StatusResult { branch, staged, modified, untracked, ahead, behind })
}

fn char_to_file_status(c: u8) -> FileStatus {
    match c {
        b'A' => FileStatus::Added,
        b'M' => FileStatus::Modified,
        b'D' => FileStatus::Deleted,
        b'R' => FileStatus::Renamed,
        b'C' => FileStatus::Copied,
        _ => FileStatus::Modified,
    }
}
