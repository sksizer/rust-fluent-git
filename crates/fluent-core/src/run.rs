//! Shared execution helpers for running CLI commands.
//!
//! Builders construct a `cmd_spec::ShellCommand`, then call these helpers
//! to execute synchronously or asynchronously.

use cmd_spec::ShellCommand;
use std::process::Output;

/// Execute a ShellCommand synchronously, returning raw Output.
#[cfg(feature = "blocking")]
pub fn run_sync(cmd: &ShellCommand) -> Result<Output, std::io::Error> {
    let mut process = std::process::Command::from(cmd);
    process.output()
}

/// Execute a ShellCommand asynchronously, returning raw Output.
#[cfg(feature = "tokio")]
pub async fn run_async(cmd: &ShellCommand) -> Result<Output, std::io::Error> {
    let mut process = tokio::process::Command::from(cmd);
    process.output().await
}

/// Extract trimmed stdout from Output.
pub fn stdout_string(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Extract trimmed stderr from Output.
pub fn stderr_string(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}
