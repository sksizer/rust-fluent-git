//! Builder for `limactl copy` operations.

#![allow(dead_code)]

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::CopyError;
use fluent_core::{CommandError, stderr_string, stdout_string};

// ── CopyBuilder ─────────────────────────────────────────────────────

pub struct CopyBuilder {
    source: String,
    target: String,
    recursive: bool,
    backend: Option<String>,
    verbose: bool,
}

impl CopyBuilder {
    pub(crate) fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self { source: source.into(), target: target.into(), recursive: false, backend: None, verbose: false }
    }

    /// Enable recursive copy.
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    /// Set the copy backend ("scp", "rsync", or "auto").
    pub fn backend(mut self, backend: impl Into<String>) -> Self {
        self.backend = Some(backend.into());
        self
    }

    /// Enable verbose output.
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Extract an instance name from the source or target spec.
    ///
    /// Returns the portion before `:` if the spec looks like `INSTANCE:/path`.
    pub(crate) fn instance_name_hint(&self) -> String {
        for spec in [&self.source, &self.target] {
            if let Some(name) = spec.split(':').next()
                && !name.is_empty()
                && !name.starts_with('/')
                && !name.starts_with('.')
            {
                return name.to_string();
            }
        }
        "unknown".to_string()
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("limactl").arg("copy").arg("--tty=false");

        if self.recursive {
            cmd = cmd.arg("-r");
        }

        if let Some(ref backend) = self.backend {
            cmd = cmd.arg(format!("--backend={backend}"));
        }

        if self.verbose {
            cmd = cmd.arg("-v");
        }

        cmd = cmd.arg(&self.source).arg(&self.target);

        cmd
    }
}

// ── Output Parsing ──────────────────────────────────────────────────

pub(crate) fn parse_copy_output(output: &Output, name: &str) -> Result<(), CopyError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("does not exist") || lower.contains("not found") {
        return Err(CopyError::NotFound { name: name.to_string() });
    }

    if lower.contains("not running") {
        return Err(CopyError::NotRunning { name: name.to_string() });
    }

    Err(CopyError::Command(CommandError::Failed {
        args: "limactl copy".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
