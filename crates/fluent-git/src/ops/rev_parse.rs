//! Builder for `git rev-parse` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::RevParseError;
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

/// Builder for a `git rev-parse` command.
pub struct RevParseBuilder<'a> {
    repo_path: &'a Path,
    reference: String,
}

impl<'a> RevParseBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path, reference: impl Into<String>) -> Self {
        Self { repo_path, reference: reference.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("rev-parse")
            .arg(self.reference.as_str())
    }

    pub(crate) fn reference(&self) -> &str {
        &self.reference
    }
}

/// Parse rev-parse output.
pub(crate) fn parse_rev_parse_output(output: &Output, reference: &str) -> Result<String, RevParseError> {
    if output.status.success() {
        return Ok(stdout_string(output));
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    if stderr.contains("unknown revision") || stderr.contains("bad revision") || stderr.contains("ambiguous argument") {
        return Err(RevParseError::RefNotFound { reference: reference.to_string() });
    }

    Err(RevParseError::Command(CommandError::Failed {
        args: format!("rev-parse {reference}"),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}
