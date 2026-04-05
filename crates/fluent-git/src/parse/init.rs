//! Shared parsing logic for `git init` output.
//!
//! Used by both sync and async implementations.

use std::path::{Path, PathBuf};
use std::process::Output;

use fluent_core::CommandError;

use crate::error::InitError;
use crate::types::InitResult;
use fluent_core::{stderr_string, stdout_string};

/// Parse the output of `git init` into an `InitResult`.
///
/// The `path` and `bare` arguments come from the builder (they are known
/// before the command runs). The `branch` is extracted from the command
/// output or falls back to the explicitly-requested branch name.
pub fn parse_init_output(
    output: Output,
    path: PathBuf,
    bare: bool,
    requested_branch: Option<&str>,
) -> Result<InitResult, InitError> {
    let stdout = stdout_string(&output);
    let stderr = stderr_string(&output);

    if !output.status.success() {
        return Err(classify_error(stderr, &path, requested_branch));
    }

    // On success, git prints one of:
    //   Initialized empty Git repository in /path/.git/
    //   Reinitialized existing Git repository in /path/.git/
    // When `-b <name>` is used the output is identical — the branch only
    // shows up inside the repo, not in stdout.
    let branch = requested_branch
        .map(String::from)
        .or_else(|| extract_branch_from_output(&stdout))
        .unwrap_or_else(|| "main".to_string());

    Ok(InitResult { path, branch, bare })
}

/// Try to extract a branch name from `git init` stdout.
///
/// Git >= 2.28 with `init.defaultBranch` may print:
///   `Initialized empty Git repository in /path/.git/`
/// There is no branch name in the output itself. Newer git versions
/// (2.30+) with `--initial-branch` echo the branch in some locales,
/// but this is unreliable. We return `None` and let the caller fall
/// back to "main" or the requested branch.
fn extract_branch_from_output(_stdout: &str) -> Option<String> {
    // Git's init output does not reliably contain the branch name.
    // The caller should use the `-b` flag value if available, or
    // default to "main".
    None
}

/// Classify a failed `git init` into a specific error variant.
fn classify_error(stderr: String, path: &Path, requested_branch: Option<&str>) -> InitError {
    let lower = stderr.to_lowercase();

    if lower.contains("permission denied") {
        return InitError::PermissionDenied { path: path.to_path_buf() };
    }

    if lower.contains("invalid branch name") || lower.contains("not a valid branch name") {
        let name = requested_branch.unwrap_or("<unknown>").to_string();
        return InitError::InvalidBranchName { name, reason: stderr.clone() };
    }

    if lower.contains("already exists") {
        return InitError::AlreadyExists { path: path.to_path_buf() };
    }

    // Fall through: generic command error.
    InitError::Command(CommandError::Failed {
        args: format!("init {}", path.display()),
        code: -1,
        stdout: String::new(),
        stderr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    fn success_output(stdout: &str) -> Output {
        Output { status: ExitStatus::from_raw(0), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() }
    }

    fn failure_output(stderr: &str) -> Output {
        Output {
            status: ExitStatus::from_raw(256), // exit code 1
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    #[test]
    fn parse_success_default_branch() {
        let output = success_output("Initialized empty Git repository in /tmp/test/.git/\n");
        let result = parse_init_output(output, PathBuf::from("/tmp/test"), false, None).unwrap();
        assert_eq!(result.path, PathBuf::from("/tmp/test"));
        assert_eq!(result.branch, "main");
        assert!(!result.bare);
    }

    #[test]
    fn parse_success_explicit_branch() {
        let output = success_output("Initialized empty Git repository in /tmp/test/.git/\n");
        let result = parse_init_output(output, PathBuf::from("/tmp/test"), false, Some("develop")).unwrap();
        assert_eq!(result.branch, "develop");
    }

    #[test]
    fn parse_success_bare() {
        let output = success_output("Initialized empty Git repository in /tmp/test/\n");
        let result = parse_init_output(output, PathBuf::from("/tmp/test"), true, None).unwrap();
        assert!(result.bare);
    }

    #[test]
    fn parse_permission_denied() {
        let output = failure_output("fatal: cannot mkdir /root/test: Permission denied\n");
        let err = parse_init_output(output, PathBuf::from("/root/test"), false, None).unwrap_err();
        assert!(matches!(err, InitError::PermissionDenied { .. }));
    }

    #[test]
    fn parse_invalid_branch_name() {
        let output = failure_output("fatal: 'bad..name' is not a valid branch name\n");
        let err = parse_init_output(output, PathBuf::from("/tmp/test"), false, Some("bad..name")).unwrap_err();
        assert!(matches!(err, InitError::InvalidBranchName { .. }));
    }
}
