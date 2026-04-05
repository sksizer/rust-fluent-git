//! Builder for `git checkout` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CheckoutError, CommandError};
use crate::run::{stderr_string, stdout_string};

/// Entry point builder for checkout operations.
pub struct CheckoutBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> CheckoutBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Checkout an existing branch.
    pub fn branch(self, name: impl Into<String>) -> CheckoutBranchBuilder<'a> {
        CheckoutBranchBuilder { repo_path: self.repo_path, name: name.into(), create: false }
    }

    /// Create and checkout a new branch.
    pub fn new_branch(self, name: impl Into<String>) -> CheckoutBranchBuilder<'a> {
        CheckoutBranchBuilder { repo_path: self.repo_path, name: name.into(), create: true }
    }
}

pub struct CheckoutBranchBuilder<'a> {
    repo_path: &'a Path,
    name: String,
    create: bool,
}

impl<'a> CheckoutBranchBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("checkout");

        if self.create {
            cmd = cmd.arg("-b");
        }

        cmd.arg(&self.name)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn is_create(&self) -> bool {
        self.create
    }
}

pub(crate) fn parse_checkout_output(output: &Output, name: &str, create: bool) -> Result<(), CheckoutError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("did not match") || lower.contains("pathspec") && lower.contains("did not match") {
        return Err(CheckoutError::RefNotFound { reference: name.to_string() });
    }

    if lower.contains("already exists") {
        return Err(CheckoutError::BranchAlreadyExists { name: name.to_string() });
    }

    if lower.contains("overwritten by checkout") || lower.contains("uncommitted changes") {
        // Try to extract file names from the error message
        let files: Vec<String> = stderr.lines().filter(|l| l.starts_with('\t')).map(|l| l.trim().to_string()).collect();
        return Err(CheckoutError::UncommittedChanges { files });
    }

    let code = output.status.code().unwrap_or(-1);
    let action = if create { "checkout -b" } else { "checkout" };
    Err(CheckoutError::Command(CommandError::Failed {
        args: format!("{action} {name}"),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}
