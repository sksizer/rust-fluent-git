//! Parse logic for `git rev-parse --git-dir` used by the `open` operation.

use std::path::Path;
use std::process::Output;

use fluent_core::CommandError;

use crate::error::OpenError;
use fluent_core::{stderr_string, stdout_string};

/// Validate the output of `git -C <path> rev-parse --git-dir`.
///
/// On success returns the trimmed stdout (the git-dir path).
/// On failure classifies the error into an [`OpenError`] variant.
pub fn parse_open(output: &Output, path: &Path) -> Result<String, OpenError> {
    if output.status.success() {
        return Ok(stdout_string(output));
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    if stderr.contains("not a git repository") {
        return Err(OpenError::NotARepo { path: path.to_path_buf() });
    }

    if stderr.contains("Permission denied")
        || stderr.contains("permission denied")
        || stderr.contains("not accessible")
        || stderr.contains("No such file or directory")
    {
        return Err(OpenError::NotAccessible { path: path.to_path_buf(), reason: stderr.clone() });
    }

    if stderr.contains("corrupt") || stderr.contains("broken") {
        return Err(OpenError::CorruptRepo { path: path.to_path_buf(), reason: stderr.clone() });
    }

    // Fallback: unrecognised non-zero exit
    Err(OpenError::Command(CommandError::Failed {
        args: format!("-C {} rev-parse --git-dir", path.display()),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}
