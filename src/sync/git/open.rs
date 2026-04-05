//! Synchronous `open` — verify a path is a git repository and return a `Repo`.

use std::path::Path;

use cmd_spec::ShellCommand;

use crate::error::OpenError;
use crate::parse::open::parse_open;
use crate::run::run_sync;
use crate::types::Repo;

/// Open an existing git repository at `path`.
///
/// Runs `git -C <path> rev-parse --git-dir` to verify the path is a valid
/// repository, then returns a [`Repo`] whose `path()` is the canonicalized
/// absolute path.
pub fn open(path: impl AsRef<Path>) -> Result<Repo, OpenError> {
    let path = path.as_ref();

    // Convert to absolute path without canonicalizing (avoids symlink resolution).
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(OpenError::Io)?
            .join(path)
    };

    // Check accessibility before running git.
    if !abs_path.exists() {
        return Err(OpenError::NotAccessible {
            path: abs_path,
            reason: "path does not exist".to_string(),
        });
    }

    let cmd = ShellCommand::new("git")
        .arg("-C")
        .arg(abs_path.to_string_lossy().as_ref())
        .arg("rev-parse")
        .arg("--git-dir");

    let output = run_sync(&cmd)?;

    parse_open(&output, &abs_path)?;

    Ok(Repo::new(abs_path))
}
