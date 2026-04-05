//! Builder for `git commit` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommitError, CommandError};
use crate::run::{stderr_string, stdout_string};
use crate::types::{Author, CommitResult};

/// Builder for a `git commit` command.
pub struct CommitBuilder<'a> {
    repo_path: &'a Path,
    message: Option<String>,
    author_name: Option<String>,
    author_email: Option<String>,
    allow_empty: bool,
    amend: bool,
    no_verify: bool,
}

impl<'a> CommitBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self {
            repo_path,
            message: None,
            author_name: None,
            author_email: None,
            allow_empty: false,
            amend: false,
            no_verify: false,
        }
    }

    /// Set the commit message.
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    /// Set the author name and email.
    pub fn author(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.author_name = Some(name.into());
        self.author_email = Some(email.into());
        self
    }

    /// Allow creating an empty commit.
    pub fn allow_empty(mut self) -> Self {
        self.allow_empty = true;
        self
    }

    /// Amend the previous commit.
    pub fn amend(mut self) -> Self {
        self.amend = true;
        self
    }

    /// Skip pre-commit and commit-msg hooks.
    pub fn no_verify(&mut self) -> &mut Self {
        self.no_verify = true;
        self
    }

    /// Scoped mutation via closure.
    pub fn with(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }

    pub(crate) fn build_commit_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("commit");

        if let Some(ref msg) = self.message {
            cmd = cmd.arg("-m").arg(msg.as_str());
        }

        if let (Some(name), Some(email)) = (&self.author_name, &self.author_email) {
            cmd = cmd.arg("--author").arg(format!("{name} <{email}>"));
        }

        if self.allow_empty {
            cmd = cmd.arg("--allow-empty");
        }

        if self.amend {
            cmd = cmd.arg("--amend");
        }

        if self.no_verify {
            cmd = cmd.arg("--no-verify");
        }

        cmd
    }

    /// Build the command to query commit details after a successful commit.
    pub(crate) fn build_show_command(&self) -> ShellCommand {
        // Use git log to get the latest commit info in a parseable format
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("log")
            .arg("-1")
            .arg("--format=%H%n%h%n%s%n%D%n%an%n%ae%n")
            .arg("--shortstat")
    }

}

/// Check if the commit command succeeded; classify errors if it didn't.
pub(crate) fn parse_commit_output(output: &Output) -> Result<(), CommitError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let stdout = stdout_string(output);
    let combined = format!("{stderr} {stdout}");
    let lower = combined.to_lowercase();

    if lower.contains("nothing to commit") || lower.contains("nothing added to commit") {
        return Err(CommitError::NothingToCommit);
    }

    if lower.contains("empty message") || lower.contains("aborting commit due to empty commit message") {
        return Err(CommitError::EmptyMessage);
    }

    if lower.contains("please tell me who you are")
        || lower.contains("user.name")
        || lower.contains("committer identity")
    {
        return Err(CommitError::IdentityNotConfigured);
    }

    if lower.contains("gpg") && lower.contains("failed") {
        return Err(CommitError::SigningFailed {
            reason: stderr.clone(),
        });
    }

    let code = output.status.code().unwrap_or(-1);
    Err(CommitError::Command(CommandError::Failed {
        args: "commit".to_string(),
        code,
        stdout,
        stderr,
    }))
}

/// Parse the output of `git log -1 --format=... --shortstat` to extract commit details.
pub(crate) fn parse_commit_details(output: &Output) -> Result<CommitResult, CommitError> {
    let stdout = stdout_string(output);
    let lines: Vec<&str> = stdout.lines().collect();

    // Format: %H\n%h\n%s\n%D\n%an\n%ae\n followed by shortstat line
    if lines.len() < 6 {
        return Err(CommitError::Command(CommandError::Failed {
            args: "log -1".to_string(),
            code: -1,
            stdout: stdout.clone(),
            stderr: stderr_string(output),
        }));
    }

    let sha = lines[0].to_string();
    let short_sha = lines[1].to_string();
    let message = lines[2].to_string();
    let refs_line = lines[3];
    let author_name = lines[4].to_string();
    let author_email = lines[5].to_string();

    // Extract branch from refs (e.g., "HEAD -> main, origin/main")
    let branch = extract_branch_from_refs(refs_line);

    // Parse shortstat line if present (e.g., " 1 file changed, 1 insertion(+)")
    let (files_changed, insertions, deletions) = lines
        .iter()
        .skip(6)
        .find(|l| l.contains("changed") || l.contains("insertion") || l.contains("deletion"))
        .map(|l| parse_shortstat(l))
        .unwrap_or((0, 0, 0));

    Ok(CommitResult {
        sha,
        short_sha,
        message,
        branch,
        author: Author {
            name: author_name,
            email: author_email,
        },
        files_changed,
        insertions,
        deletions,
    })
}

fn extract_branch_from_refs(refs_line: &str) -> String {
    // refs_line looks like "HEAD -> main" or "HEAD -> main, origin/main" or ""
    if let Some(arrow_pos) = refs_line.find("-> ") {
        let after_arrow = &refs_line[arrow_pos + 3..];
        // Take until comma or end
        after_arrow
            .split(',')
            .next()
            .unwrap_or("HEAD")
            .trim()
            .to_string()
    } else {
        "HEAD".to_string()
    }
}

fn parse_shortstat(line: &str) -> (usize, usize, usize) {
    let mut files_changed = 0;
    let mut insertions = 0;
    let mut deletions = 0;

    // Parse: " 1 file changed, 2 insertions(+), 1 deletion(-)"
    for part in line.split(',') {
        let part = part.trim();
        if let Some(num_str) = part.split_whitespace().next() {
            if let Ok(num) = num_str.parse::<usize>() {
                if part.contains("file") {
                    files_changed = num;
                } else if part.contains("insertion") {
                    insertions = num;
                } else if part.contains("deletion") {
                    deletions = num;
                }
            }
        }
    }

    (files_changed, insertions, deletions)
}
