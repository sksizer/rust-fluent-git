//! Asynchronous `git init` builder (requires `tokio` feature).

use std::path::{Path, PathBuf};

use cmd_spec::ShellCommand;

use crate::error::InitError;
use crate::parse::init::parse_init_output;
use crate::run::run_async;
use crate::types::InitResult;

/// Create an `AsyncInitBuilder` for the given directory path.
///
/// ```no_run
/// # use std::path::Path;
/// # async fn example() {
/// # use fluent_git::git::init;
/// let result = init(Path::new("/tmp/my-repo"))
///     .initial_branch("main")
///     .run()
///     .await
///     .unwrap();
/// println!("Initialized at {:?}, branch: {}", result.path, result.branch);
/// # }
/// ```
pub fn init(path: &Path) -> AsyncInitBuilder {
    AsyncInitBuilder {
        path: path.to_path_buf(),
        bare: false,
        branch: None,
    }
}

/// Builder for an asynchronous `git init` command.
pub struct AsyncInitBuilder {
    path: PathBuf,
    bare: bool,
    branch: Option<String>,
}

impl AsyncInitBuilder {
    /// Create a bare repository.
    pub fn bare(mut self) -> Self {
        self.bare = true;
        self
    }

    /// Set the initial branch name (equivalent to `git init -b <name>`).
    pub fn initial_branch(mut self, name: impl Into<String>) -> Self {
        self.branch = Some(name.into());
        self
    }

    /// Build the `ShellCommand` without running it.
    fn build_command(&self) -> ShellCommand {
        let cmd = ShellCommand::new("git").arg("init");

        let cmd = if self.bare { cmd.arg("--bare") } else { cmd };

        let cmd = match &self.branch {
            Some(name) => cmd.arg("-b").arg(name.as_str()),
            None => cmd,
        };

        cmd.arg(self.path.to_string_lossy().as_ref())
    }

    /// Execute the `git init` command asynchronously.
    pub async fn run(self) -> Result<InitResult, InitError> {
        let cmd = self.build_command();
        let output = run_async(&cmd).await?;
        parse_init_output(output, self.path, self.bare, self.branch.as_deref())
    }
}
