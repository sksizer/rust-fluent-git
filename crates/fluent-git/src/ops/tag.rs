//! Builder for `git tag` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::TagError;
use crate::types::{Author, TagInfo};
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

/// Entry point builder for tag operations.
pub struct TagBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> TagBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Create a new tag.
    pub fn create(self, name: impl Into<String>) -> TagCreateBuilder<'a> {
        TagCreateBuilder { repo_path: self.repo_path, name: name.into(), message: None }
    }

    /// Delete a tag.
    pub fn delete(self, name: impl Into<String>) -> TagDeleteBuilder<'a> {
        TagDeleteBuilder { repo_path: self.repo_path, name: name.into() }
    }

    /// List all tags.
    pub fn list(self) -> TagListBuilder<'a> {
        TagListBuilder { repo_path: self.repo_path }
    }
}

// ── Create ─────────────────────────────────────────────────────────────

pub struct TagCreateBuilder<'a> {
    repo_path: &'a Path,
    name: String,
    message: Option<String>,
}

impl<'a> TagCreateBuilder<'a> {
    /// Set a message, making this an annotated tag.
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("tag");

        if let Some(ref msg) = self.message {
            cmd = cmd.arg("-a").arg("-m").arg(msg.as_str());
        }

        cmd = cmd.arg(&self.name);

        cmd
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_create_output(output: &Output, name: &str) -> Result<(), TagError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("already exists") {
        return Err(TagError::AlreadyExists { name: name.to_string() });
    }

    if lower.contains("not a valid tag name") || lower.contains("invalid tag name") {
        return Err(TagError::InvalidName { name: name.to_string(), reason: stderr.clone() });
    }

    Err(TagError::Command(CommandError::Failed {
        args: format!("tag {name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Delete ─────────────────────────────────────────────────────────────

pub struct TagDeleteBuilder<'a> {
    repo_path: &'a Path,
    name: String,
}

impl<'a> TagDeleteBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("tag")
            .arg("-d")
            .arg(&self.name)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str) -> Result<(), TagError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") {
        return Err(TagError::NotFound { name: name.to_string() });
    }

    Err(TagError::Command(CommandError::Failed {
        args: format!("tag -d {name}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── List ───────────────────────────────────────────────────────────────

pub struct TagListBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> TagListBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("tag")
            .arg("-l")
            .arg("--format=%(refname:short)%09%(objectname:short)%09%(objecttype)%09%(taggername)%09%(taggeremail)%09%(contents:subject)")
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<TagInfo>, TagError> {
    if !output.status.success() {
        let stderr = stderr_string(output);
        return Err(TagError::Command(CommandError::Failed {
            args: "tag -l".to_string(),
            code: output.status.code().unwrap_or(-1),
            stdout: stdout_string(output),
            stderr,
        }));
    }

    let stdout = stdout_string(output);
    let mut tags = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Format: name\tsha\tobjecttype\ttaggername\ttaggeremail\tsubject
        let parts: Vec<&str> = line.splitn(6, '\t').collect();
        if parts.len() < 3 {
            continue;
        }

        let name = parts[0].to_string();
        let sha = parts[1].to_string();
        let object_type = parts[2];
        let annotated = object_type == "tag";

        let tagger = if parts.len() >= 5 {
            let tagger_name = parts[3].trim();
            let tagger_email = parts[4].trim().trim_start_matches('<').trim_end_matches('>');
            if !tagger_name.is_empty() && !tagger_email.is_empty() {
                Some(Author { name: tagger_name.to_string(), email: tagger_email.to_string() })
            } else {
                None
            }
        } else {
            None
        };

        let message = if parts.len() >= 6 {
            let msg = parts[5].trim();
            if msg.is_empty() { None } else { Some(msg.to_string()) }
        } else {
            None
        };

        tags.push(TagInfo { name, sha, message, tagger, annotated });
    }

    Ok(tags)
}
