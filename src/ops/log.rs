//! Builder for `git log` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, LogError};
use crate::run::{stderr_string, stdout_string};
use crate::types::{Author, LogEntry};

/// Separator used between fields in the log format.
const SEP: &str = "\x1e";
/// Separator used between records in the log format.
const RECORD_SEP: &str = "\x1f";

/// Builder for a `git log` command.
pub struct LogBuilder<'a> {
    repo_path: &'a Path,
    max_count: Option<usize>,
    oneline: bool,
    author: Option<String>,
}

impl<'a> LogBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self {
            repo_path,
            max_count: None,
            oneline: false,
            author: None,
        }
    }

    /// Limit the number of commits returned.
    pub fn max_count(mut self, n: usize) -> Self {
        self.max_count = Some(n);
        self
    }

    /// Use oneline format (short sha + message only).
    pub fn oneline(mut self) -> Self {
        self.oneline = true;
        self
    }

    /// Filter by author name/email.
    pub fn author(mut self, name: impl Into<String>) -> Self {
        self.author = Some(name.into());
        self
    }

    /// Scoped mutation via closure.
    pub fn with(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let format_str = format!(
            "%H{SEP}%h{SEP}%s{SEP}%D{SEP}%an{SEP}%ae{SEP}%aI{RECORD_SEP}"
        );

        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("log")
            .arg(format!("--format={format_str}"));

        if let Some(n) = self.max_count {
            cmd = cmd.arg(format!("-n{n}"));
        }

        if let Some(ref author) = self.author {
            cmd = cmd.arg(format!("--author={author}"));
        }

        cmd
    }
}

pub(crate) fn parse_log_output(output: &Output) -> Result<Vec<LogEntry>, LogError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        let code = output.status.code().unwrap_or(-1);

        if stderr.contains("does not have any commits")
            || stderr.contains("unknown revision")
            || stderr.contains("bad default revision")
        {
            return Err(LogError::NoCommits);
        }

        return Err(LogError::Command(CommandError::Failed {
            args: "log".to_string(),
            code,
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);

    if stdout.is_empty() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();

    for record in stdout.split(RECORD_SEP) {
        let record = record.trim();
        if record.is_empty() {
            continue;
        }

        let fields: Vec<&str> = record.split(SEP).collect();
        if fields.len() < 7 {
            continue;
        }

        let refs: Vec<String> = fields[3]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        entries.push(LogEntry {
            sha: fields[0].to_string(),
            short_sha: fields[1].to_string(),
            message: fields[2].to_string(),
            author: Author {
                name: fields[4].to_string(),
                email: fields[5].to_string(),
            },
            timestamp: fields[6].to_string(),
            refs,
        });
    }

    Ok(entries)
}
