//! Synchronous `git init` builder.

use std::path::{Path, PathBuf};

use cmd_spec::ShellCommand;

use crate::error::InitError;
use crate::parse::init::parse_init_output;
use crate::types::InitResult;
use fluent_core::run_sync;

/// Create an `InitBuilder` for the given directory path.
///
/// ```no_run
/// # use std::path::Path;
/// # use fluent_git::sync::git::init;
/// let result = init(Path::new("/tmp/my-repo"))
///     .initial_branch("main")
///     .run()
///     .unwrap();
/// println!("Initialized at {:?}, branch: {}", result.path, result.branch);
/// ```
pub fn init(path: &Path) -> InitBuilder {
    InitBuilder { path: path.to_path_buf(), bare: false, branch: None }
}

/// Builder for a synchronous `git init` command.
pub struct InitBuilder {
    path: PathBuf,
    bare: bool,
    branch: Option<String>,
}

impl InitBuilder {
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

    /// Execute the `git init` command synchronously.
    pub fn run(self) -> Result<InitResult, InitError> {
        let cmd = self.build_command();
        let output = run_sync(&cmd)?;
        parse_init_output(output, self.path, self.bare, self.branch.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_simple_command() {
        let builder = init(Path::new("/tmp/test"));
        let cmd = builder.build_command();
        assert_eq!(cmd.program, "git");
        assert_eq!(cmd.args, vec!["init", "/tmp/test"]);
    }

    #[test]
    fn build_bare_command() {
        let builder = init(Path::new("/tmp/test")).bare();
        let cmd = builder.build_command();
        assert_eq!(cmd.args, vec!["init", "--bare", "/tmp/test"]);
    }

    #[test]
    fn build_branch_command() {
        let builder = init(Path::new("/tmp/test")).initial_branch("develop");
        let cmd = builder.build_command();
        assert_eq!(cmd.args, vec!["init", "-b", "develop", "/tmp/test"]);
    }

    #[test]
    fn build_bare_with_branch() {
        let builder = init(Path::new("/tmp/test")).bare().initial_branch("trunk");
        let cmd = builder.build_command();
        assert_eq!(cmd.args, vec!["init", "--bare", "-b", "trunk", "/tmp/test"]);
    }
}
