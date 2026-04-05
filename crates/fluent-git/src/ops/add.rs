//! Builder for `git add` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::AddError;
use crate::types::AddResult;
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

/// Builder for a `git add` command.
pub struct AddBuilder<'a> {
    repo_path: &'a Path,
    paths: Vec<String>,
    all: bool,
}

impl<'a> AddBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path, paths: Vec::new(), all: false }
    }

    /// Add a specific file path to stage.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.paths.push(path.into());
        self
    }

    /// Stage all changes (equivalent to `git add --all`).
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// Scoped mutation via closure.
    pub fn mutate(self) -> AddBuilderMut<'a> {
        AddBuilderMut { inner: self }
    }

    pub(crate) fn build_add_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("add");

        if self.all {
            cmd = cmd.arg("--all");
        }

        for p in &self.paths {
            cmd = cmd.arg(p.as_str());
        }

        cmd
    }

    pub(crate) fn build_diff_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("diff")
            .arg("--cached")
            .arg("--name-only")
    }

    pub(crate) fn paths_ref(&self) -> &[String] {
        &self.paths
    }
}

/// Mutable handle for `AddBuilder`.
pub struct AddBuilderMut<'a> {
    inner: AddBuilder<'a>,
}

impl<'a> AddBuilderMut<'a> {
    pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
        self.inner.paths.push(path.into());
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.inner.all = true;
        self
    }

    pub fn finish(self) -> AddBuilder<'a> {
        self.inner
    }
}

pub(crate) fn parse_add_output(output: &Output, paths: &[String]) -> Result<(), AddError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    if stderr.contains("did not match any files") {
        let path = paths.first().cloned().unwrap_or_default();
        return Err(AddError::PathNotFound { path });
    }

    if stderr.contains("outside repository") {
        let path = paths.first().cloned().unwrap_or_default();
        return Err(AddError::OutsideRepo { path });
    }

    Err(AddError::Command(CommandError::Failed {
        args: "add".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

pub(crate) fn parse_staged_files(output: &Output) -> AddResult {
    let stdout = stdout_string(output);
    let files: Vec<String> = stdout.lines().filter(|l| !l.is_empty()).map(|l| l.to_string()).collect();
    AddResult { files }
}
