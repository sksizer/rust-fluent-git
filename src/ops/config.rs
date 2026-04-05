//! Builder for `git config` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{ConfigError, CommandError};
use crate::run::{stderr_string, stdout_string};

/// Entry point builder for config operations.
pub struct ConfigBuilder<'a> {
    repo_path: &'a Path,
}

impl<'a> ConfigBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Set a config key to a value.
    pub fn set(self, key: impl Into<String>, value: impl Into<String>) -> ConfigSetBuilder<'a> {
        ConfigSetBuilder {
            repo_path: self.repo_path,
            key: key.into(),
            value: value.into(),
        }
    }

    /// Get a config value by key.
    pub fn get(self, key: impl Into<String>) -> ConfigGetBuilder<'a> {
        ConfigGetBuilder {
            repo_path: self.repo_path,
            key: key.into(),
        }
    }

    /// Unset a config key.
    pub fn unset(self, key: impl Into<String>) -> ConfigUnsetBuilder<'a> {
        ConfigUnsetBuilder {
            repo_path: self.repo_path,
            key: key.into(),
        }
    }
}

// ── Set ─────────────────────────────────────────────────────────────────

pub struct ConfigSetBuilder<'a> {
    repo_path: &'a Path,
    key: String,
    value: String,
}

impl<'a> ConfigSetBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("config")
            .arg("--local")
            .arg(&self.key)
            .arg(&self.value)
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }
}

pub(crate) fn parse_config_set_output(output: &Output, key: &str) -> Result<(), ConfigError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("invalid key") || lower.contains("invalid config key") {
        return Err(ConfigError::InvalidKey {
            key: key.to_string(),
            reason: stderr.clone(),
        });
    }

    if lower.contains("locked") || lower.contains("lock") && lower.contains("failed") {
        return Err(ConfigError::Locked);
    }

    Err(ConfigError::Command(CommandError::Failed {
        args: format!("config {key}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Get ─────────────────────────────────────────────────────────────────

pub struct ConfigGetBuilder<'a> {
    repo_path: &'a Path,
    key: String,
}

impl<'a> ConfigGetBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("config")
            .arg("--local")
            .arg(&self.key)
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }
}

pub(crate) fn parse_config_get_output(output: &Output, key: &str) -> Result<String, ConfigError> {
    if output.status.success() {
        return Ok(stdout_string(output));
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    // git config returns exit code 1 when key is not found
    if code == 1 && stderr.is_empty() {
        return Err(ConfigError::KeyNotFound {
            key: key.to_string(),
        });
    }

    Err(ConfigError::Command(CommandError::Failed {
        args: format!("config {key}"),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Unset ───────────────────────────────────────────────────────────────

pub struct ConfigUnsetBuilder<'a> {
    repo_path: &'a Path,
    key: String,
}

impl<'a> ConfigUnsetBuilder<'a> {
    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("config")
            .arg("--local")
            .arg("--unset")
            .arg(&self.key)
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }
}

pub(crate) fn parse_config_unset_output(output: &Output, key: &str) -> Result<(), ConfigError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    // git config --unset returns exit code 5 when key is not found
    if code == 5 || (code == 1 && stderr.is_empty()) {
        return Err(ConfigError::KeyNotFound {
            key: key.to_string(),
        });
    }

    Err(ConfigError::Command(CommandError::Failed {
        args: format!("config --unset {key}"),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}
