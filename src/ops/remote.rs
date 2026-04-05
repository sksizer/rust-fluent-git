//! Builder for `git remote` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, RemoteError};
use crate::run::{stderr_string, stdout_string};
use crate::types::RemoteInfo;

/// Entry point builder for remote operations.
pub struct RemoteBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> RemoteBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Add a new remote.
    pub fn add(self, name: impl Into<String>, url: impl Into<String>) -> RemoteAddBuilder<'a> {
        RemoteAddBuilder { repo_path: self.repo_path, name: name.into(), url: url.into() }
    }

    /// Remove an existing remote.
    pub fn remove(self, name: impl Into<String>) -> RemoteRemoveBuilder<'a> {
        RemoteRemoveBuilder { repo_path: self.repo_path, name: name.into() }
    }

    /// List all remotes.
    pub fn list(self) -> RemoteListBuilder<'a> {
        RemoteListBuilder { repo_path: self.repo_path }
    }
}

// ── Add ────────────────────────────────────────────────────────────────

pub struct RemoteAddBuilder<'a> {
    repo_path: &'a Path,
    name: String,
    url: String,
}

impl<'a> RemoteAddBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("remote")
            .arg("add")
            .arg(&self.name)
            .arg(&self.url)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_add_output(output: &Output, name: &str) -> Result<(), RemoteError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(RemoteError::AlreadyExists { name: name.to_string() });
    }

    Err(RemoteError::Command(CommandError::Failed {
        args: format!("remote add {name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Remove ─────────────────────────────────────────────────────────────

pub struct RemoteRemoveBuilder<'a> {
    repo_path: &'a Path,
    name: String,
}

impl<'a> RemoteRemoveBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("remote")
            .arg("remove")
            .arg(&self.name)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_remove_output(output: &Output, name: &str) -> Result<(), RemoteError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("no such remote") || lower.contains("not found") {
        return Err(RemoteError::NotFound { name: name.to_string() });
    }

    Err(RemoteError::Command(CommandError::Failed {
        args: format!("remote remove {name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ───────────────────────────────────────────────────────────────

pub struct RemoteListBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> RemoteListBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("remote").arg("-v")
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<RemoteInfo>, RemoteError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        return Err(RemoteError::Command(CommandError::Failed {
            args: "remote -v".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut remotes: Vec<RemoteInfo> = Vec::new();

    // Format: "origin\thttps://... (fetch)" and "origin\thttps://... (push)"
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let name = parts[0].to_string();
        let url = parts[1].to_string();
        let kind = parts[2]; // "(fetch)" or "(push)"

        // Find or create the remote entry
        if let Some(existing) = remotes.iter_mut().find(|r| r.name == name) {
            if kind == "(fetch)" {
                existing.fetch_url = url;
            } else if kind == "(push)" {
                existing.push_url = url;
            }
        } else {
            let (fetch_url, push_url) = if kind == "(fetch)" { (url, String::new()) } else { (String::new(), url) };
            remotes.push(RemoteInfo { name, fetch_url, push_url });
        }
    }

    Ok(remotes)
}
