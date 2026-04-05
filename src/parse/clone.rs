//! Parse `git clone` output into `CloneResult` or `CloneError`.

use std::process::Output;

use crate::error::{CloneError, CommandError};
use crate::run::stderr_string;

/// Parse the output of a `git clone` invocation.
///
/// On success returns `Ok(())`. The caller constructs the `CloneResult`
/// from the builder's known configuration (remote name, depth, etc.)
/// since git clone's output doesn't reliably contain all that info.
///
/// On failure, maps stderr patterns to typed `CloneError` variants.
pub fn parse_clone_output(output: &Output, url: &str) -> Result<(), CloneError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    classify_clone_error(&stderr, url, code)
}

/// Build the `ShellCommand` for detecting the current branch of a cloned repo.
///
/// Returns arguments for: `git -C <dest> symbolic-ref --short HEAD`
pub fn branch_detect_command(dest: &std::path::Path) -> cmd_spec::ShellCommand {
    cmd_spec::ShellCommand::new("git")
        .arg("-C")
        .arg(dest.to_string_lossy().as_ref())
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
}

/// Parse the output of the branch detection command.
/// Returns the branch name, or "HEAD" as fallback for detached HEAD.
pub fn parse_branch_detect(output: &Output) -> String {
    if output.status.success() { crate::run::stdout_string(output) } else { "HEAD".to_string() }
}

/// Map stderr content to a specific `CloneError` variant.
fn classify_clone_error(stderr: &str, url: &str, code: i32) -> Result<(), CloneError> {
    if stderr.contains("already exists and is not an empty directory") {
        // Extract path from: fatal: destination path 'foo' already exists ...
        let path = extract_destination_path(stderr).unwrap_or_else(|| url.into());
        return Err(CloneError::DestinationExists { path: path.into() });
    }

    // BranchNotFound must be checked before the generic "not found" pattern
    if stderr.contains("Remote branch") && stderr.contains("not found") {
        let branch = extract_branch_name(stderr).unwrap_or_default();
        return Err(CloneError::BranchNotFound { url: url.to_string(), branch });
    }

    if stderr.contains("not found") || stderr.contains("does not appear to be a git repository") {
        return Err(CloneError::RepoNotFound { url: url.to_string() });
    }

    if stderr.contains("Authentication failed") || stderr.contains("could not read Username") {
        return Err(CloneError::AuthFailed { url: url.to_string() });
    }

    if stderr.contains("Could not resolve host") || stderr.contains("unable to access") {
        return Err(CloneError::Network { url: url.to_string(), reason: stderr.to_string() });
    }

    Err(CloneError::Command(CommandError::Failed {
        args: format!("clone {url}"),
        code,
        stdout: String::new(),
        stderr: stderr.to_string(),
    }))
}

/// Extract destination path from stderr like:
/// `fatal: destination path 'some/path' already exists and is not an empty directory`
fn extract_destination_path(stderr: &str) -> Option<String> {
    let marker = "destination path '";
    let start = stderr.find(marker)? + marker.len();
    let end = stderr[start..].find('\'')?;
    Some(stderr[start..start + end].to_string())
}

/// Extract branch name from stderr like:
/// `warning: Remote branch 'foo' not found in upstream origin`
fn extract_branch_name(stderr: &str) -> Option<String> {
    // Pattern: "Remote branch 'NAME' not found" or "Remote branch NAME not found"
    let marker = "Remote branch ";
    let start = stderr.find(marker)? + marker.len();
    let rest = &stderr[start..];
    if let Some(stripped) = rest.strip_prefix('\'') {
        let end = stripped.find('\'')?;
        Some(stripped[..end].to_string())
    } else {
        let end = rest.find(' ')?;
        Some(rest[..end].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_destination_exists() {
        let err = classify_clone_error(
            "fatal: destination path 'my-repo' already exists and is not an empty directory",
            "https://github.com/user/repo.git",
            128,
        );
        match err {
            Err(CloneError::DestinationExists { path }) => {
                assert_eq!(path.to_string_lossy(), "my-repo");
            }
            other => panic!("expected DestinationExists, got {:?}", other),
        }
    }

    #[test]
    fn classify_repo_not_found() {
        let err = classify_clone_error(
            "fatal: repository 'https://github.com/user/nope.git' not found",
            "https://github.com/user/nope.git",
            128,
        );
        assert!(matches!(err, Err(CloneError::RepoNotFound { .. })));
    }

    #[test]
    fn classify_auth_failed() {
        let err = classify_clone_error(
            "fatal: Authentication failed for 'https://github.com/user/repo.git'",
            "https://github.com/user/repo.git",
            128,
        );
        assert!(matches!(err, Err(CloneError::AuthFailed { .. })));
    }

    #[test]
    fn classify_network_error() {
        let err = classify_clone_error(
            "fatal: unable to access 'https://github.com/user/repo.git': Could not resolve host: github.com",
            "https://github.com/user/repo.git",
            128,
        );
        assert!(matches!(err, Err(CloneError::Network { .. })));
    }

    #[test]
    fn classify_unknown_falls_through() {
        let err = classify_clone_error("fatal: something unexpected happened", "https://github.com/user/repo.git", 1);
        assert!(matches!(err, Err(CloneError::Command(_))));
    }

    #[test]
    fn extract_dest_path_works() {
        assert_eq!(
            extract_destination_path("fatal: destination path 'foo/bar' already exists"),
            Some("foo/bar".to_string())
        );
    }

    #[test]
    fn extract_branch_quoted() {
        assert_eq!(
            extract_branch_name("warning: Remote branch 'develop' not found in upstream origin"),
            Some("develop".to_string())
        );
    }
}
