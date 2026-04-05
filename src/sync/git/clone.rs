//! Synchronous `git clone` builder.
//!
//! Usage:
//! ```no_run
//! # use std::path::Path;
//! # use fluent_git::sync::git::clone;
//! let result = clone("https://github.com/user/repo.git")
//!     .depth(1)
//!     .branch("main")
//!     .into(Path::new("/tmp/repo"))
//!     .run()
//!     .unwrap();
//! ```

use std::path::{Path, PathBuf};

use cmd_spec::ShellCommand;

use crate::error::CloneError;
use crate::parse::clone::{branch_detect_command, parse_branch_detect, parse_clone_output};
use crate::run::run_sync;
use crate::types::CloneResult;

/// Start building a `git clone` command for the given source URL or path.
pub fn clone(source: impl AsRef<Path>) -> CloneBuilder {
    CloneBuilder {
        source: source.as_ref().to_string_lossy().to_string(),
        depth: None,
        branch: None,
        remote_name: None,
    }
}

/// Builder for `git clone` options (before destination is specified).
pub struct CloneBuilder {
    source: String,
    depth: Option<u32>,
    branch: Option<String>,
    remote_name: Option<String>,
}

impl CloneBuilder {
    /// Set the clone depth for a shallow clone.
    pub fn depth(mut self, n: u32) -> Self {
        self.depth = Some(n);
        self
    }

    /// Set the branch to check out after cloning.
    pub fn branch(mut self, name: impl Into<String>) -> Self {
        self.branch = Some(name.into());
        self
    }

    /// Set the remote name (defaults to "origin").
    pub fn remote_name(mut self, name: impl Into<String>) -> Self {
        self.remote_name = Some(name.into());
        self
    }

    /// Specify the destination directory, returning a builder that can be run.
    pub fn into(self, dest: impl AsRef<Path>) -> CloneWithDest {
        CloneWithDest {
            source: self.source,
            dest: dest.as_ref().to_path_buf(),
            depth: self.depth,
            branch: self.branch,
            remote_name: self.remote_name,
        }
    }
}

/// Builder for `git clone` with destination specified. Ready to execute.
pub struct CloneWithDest {
    source: String,
    dest: PathBuf,
    depth: Option<u32>,
    branch: Option<String>,
    remote_name: Option<String>,
}

impl CloneWithDest {
    /// Get a mutable handle for conditional argument building.
    /// Call `.finish()` on the handle to return to this builder.
    pub fn mutate(self) -> CloneWithDestMut {
        CloneWithDestMut { inner: self }
    }

    /// Execute the clone synchronously.
    pub fn run(self) -> Result<CloneResult, CloneError> {
        let cmd = self.build_command();
        let remote = self.remote_name.as_deref().unwrap_or("origin").to_string();
        let shallow = self.depth.is_some();

        let output = run_sync(&cmd).map_err(CloneError::Io)?;
        parse_clone_output(&output, &self.source)?;

        let branch = if let Some(ref b) = self.branch {
            b.clone()
        } else {
            let detect_cmd = branch_detect_command(&self.dest);
            let detect_output = run_sync(&detect_cmd).map_err(CloneError::Io)?;
            parse_branch_detect(&detect_output)
        };

        Ok(CloneResult {
            path: self.dest,
            remote,
            branch,
            shallow,
        })
    }

    fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("clone");

        if let Some(depth) = self.depth {
            cmd = cmd.arg("--depth").arg(depth.to_string());
        }
        if let Some(ref branch) = self.branch {
            cmd = cmd.arg("--branch").arg(branch.as_str());
        }
        if let Some(ref remote) = self.remote_name {
            cmd = cmd.arg("--origin").arg(remote.as_str());
        }

        cmd.arg(&self.source).arg(self.dest.to_string_lossy().as_ref())
    }
}

/// Mutable handle for `CloneWithDest`. Allows conditional modifications.
pub struct CloneWithDestMut {
    inner: CloneWithDest,
}

impl CloneWithDestMut {
    /// Set the clone depth for a shallow clone.
    pub fn depth(&mut self, n: u32) -> &mut Self {
        self.inner.depth = Some(n);
        self
    }

    /// Set the branch to check out after cloning.
    pub fn branch(&mut self, name: impl Into<String>) -> &mut Self {
        self.inner.branch = Some(name.into());
        self
    }

    /// Set the remote name (defaults to "origin").
    pub fn remote_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.inner.remote_name = Some(name.into());
        self
    }

    /// Finish mutation and return the builder.
    pub fn finish(self) -> CloneWithDest {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_simple_command() {
        let builder = clone("https://github.com/user/repo.git")
            .into(Path::new("/tmp/repo"));
        let cmd = builder.build_command();
        assert_eq!(cmd.program, "git");
        assert_eq!(
            cmd.args,
            vec!["clone", "https://github.com/user/repo.git", "/tmp/repo"]
        );
    }

    #[test]
    fn build_with_depth() {
        let builder = clone("https://github.com/user/repo.git")
            .depth(1)
            .into(Path::new("/tmp/repo"));
        let cmd = builder.build_command();
        assert_eq!(
            cmd.args,
            vec!["clone", "--depth", "1", "https://github.com/user/repo.git", "/tmp/repo"]
        );
    }

    #[test]
    fn build_with_branch() {
        let builder = clone("https://github.com/user/repo.git")
            .branch("develop")
            .into(Path::new("/tmp/repo"));
        let cmd = builder.build_command();
        assert_eq!(
            cmd.args,
            vec!["clone", "--branch", "develop", "https://github.com/user/repo.git", "/tmp/repo"]
        );
    }

    #[test]
    fn build_with_remote_name() {
        let builder = clone("https://github.com/user/repo.git")
            .remote_name("upstream")
            .into(Path::new("/tmp/repo"));
        let cmd = builder.build_command();
        assert_eq!(
            cmd.args,
            vec!["clone", "--origin", "upstream", "https://github.com/user/repo.git", "/tmp/repo"]
        );
    }

    #[test]
    fn build_full_options() {
        let builder = clone("https://github.com/user/repo.git")
            .depth(1)
            .branch("main")
            .remote_name("origin")
            .into(Path::new("/tmp/repo"));
        let cmd = builder.build_command();
        assert_eq!(
            cmd.args,
            vec![
                "clone", "--depth", "1", "--branch", "main",
                "--origin", "origin",
                "https://github.com/user/repo.git", "/tmp/repo"
            ]
        );
    }

    #[test]
    fn mutate_adds_options() {
        let mut handle = clone("https://github.com/user/repo.git")
            .into(Path::new("/tmp/repo"))
            .mutate();
        handle.depth(1).branch("main");
        let builder = handle.finish();
        let cmd = builder.build_command();
        assert_eq!(
            cmd.args,
            vec![
                "clone", "--depth", "1", "--branch", "main",
                "https://github.com/user/repo.git", "/tmp/repo"
            ]
        );
    }
}
