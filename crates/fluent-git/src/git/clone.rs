//! Asynchronous `git clone` builder (requires `tokio` feature).
//!
//! Usage:
//! ```no_run
//! # use std::path::Path;
//! # async fn example() {
//! # use fluent_git::git::clone;
//! let result = clone("https://github.com/user/repo.git")
//!     .depth(1)
//!     .branch("main")
//!     .into(Path::new("/tmp/repo"))
//!     .run()
//!     .await
//!     .unwrap();
//! # }
//! ```

use std::path::{Path, PathBuf};

use cmd_spec::ShellCommand;

use crate::error::CloneError;
use crate::parse::clone::{branch_detect_command, parse_branch_detect, parse_clone_output};
use crate::types::CloneResult;
use fluent_core::run_async;

/// Start building an async `git clone` command for the given source URL or path.
pub fn clone(source: impl AsRef<Path>) -> AsyncCloneBuilder {
    AsyncCloneBuilder {
        source: source.as_ref().to_string_lossy().to_string(),
        depth: None,
        branch: None,
        remote_name: None,
    }
}

/// Builder for async `git clone` options (before destination is specified).
pub struct AsyncCloneBuilder {
    source: String,
    depth: Option<u32>,
    branch: Option<String>,
    remote_name: Option<String>,
}

impl AsyncCloneBuilder {
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
    pub fn into(self, dest: impl AsRef<Path>) -> AsyncCloneWithDest {
        AsyncCloneWithDest {
            source: self.source,
            dest: dest.as_ref().to_path_buf(),
            depth: self.depth,
            branch: self.branch,
            remote_name: self.remote_name,
        }
    }
}

/// Async builder for `git clone` with destination specified. Ready to execute.
pub struct AsyncCloneWithDest {
    source: String,
    dest: PathBuf,
    depth: Option<u32>,
    branch: Option<String>,
    remote_name: Option<String>,
}

impl AsyncCloneWithDest {
    /// Get a mutable handle for conditional argument building.
    /// Call `.finish()` on the handle to return to this builder.
    pub fn mutate(self) -> AsyncCloneWithDestMut {
        AsyncCloneWithDestMut { inner: self }
    }

    /// Execute the clone asynchronously.
    pub async fn run(self) -> Result<CloneResult, CloneError> {
        let cmd = self.build_command();
        let remote = self.remote_name.as_deref().unwrap_or("origin").to_string();
        let shallow = self.depth.is_some();

        let output = run_async(&cmd).await.map_err(CloneError::Io)?;
        parse_clone_output(&output, &self.source)?;

        let branch = if let Some(ref b) = self.branch {
            b.clone()
        } else {
            let detect_cmd = branch_detect_command(&self.dest);
            let detect_output = run_async(&detect_cmd).await.map_err(CloneError::Io)?;
            parse_branch_detect(&detect_output)
        };

        Ok(CloneResult { path: self.dest, remote, branch, shallow })
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

/// Mutable handle for `AsyncCloneWithDest`. Allows conditional modifications.
pub struct AsyncCloneWithDestMut {
    inner: AsyncCloneWithDest,
}

impl AsyncCloneWithDestMut {
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
    pub fn finish(self) -> AsyncCloneWithDest {
        self.inner
    }
}
