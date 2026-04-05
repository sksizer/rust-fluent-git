//! Trait for discovering CLI tools on the system.

use std::path::PathBuf;

/// A CLI tool that can be discovered on the system.
pub trait CliTool {
    /// The binary name (e.g., "git", "gh", "wrangler").
    fn program() -> &'static str;

    /// Check if the tool is available on PATH.
    fn available() -> bool {
        which::which(Self::program()).is_ok()
    }

    /// Get the path to the binary.
    fn which() -> Result<PathBuf, which::Error> {
        which::which(Self::program())
    }
}
