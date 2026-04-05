//! Builder for `git diff` operations.

use std::path::Path;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::{CommandError, DiffError};
use crate::run::{stderr_string, stdout_string};
use crate::types::{DiffFile, DiffResult, DiffStats, FileStatus};

/// Builder for a `git diff` command.
pub struct DiffBuilder<'a> {
    repo_path: &'a Path,
    cached: bool,
    stat: bool,
    ref_a: Option<String>,
    ref_b: Option<String>,
}

impl<'a> DiffBuilder<'a> {
    pub(crate) fn new(repo_path: &'a Path) -> Self {
        Self { repo_path, cached: false, stat: false, ref_a: None, ref_b: None }
    }

    /// Show staged (cached) changes.
    pub fn cached(mut self) -> Self {
        self.cached = true;
        self
    }

    /// Show diffstat summary.
    pub fn stat(mut self) -> Self {
        self.stat = true;
        self
    }

    /// Compare two refs (e.g., `between("main", "feature")`).
    pub fn between(mut self, a: impl Into<String>, b: impl Into<String>) -> Self {
        self.ref_a = Some(a.into());
        self.ref_b = Some(b.into());
        self
    }

    /// Scoped mutation via closure.
    pub fn with(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }

    /// Build the numstat command for file-level stats.
    pub(crate) fn build_numstat_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git")
            .arg("-C")
            .arg(self.repo_path.to_string_lossy().as_ref())
            .arg("diff")
            .arg("--numstat");

        if self.cached {
            cmd = cmd.arg("--cached");
        }

        if let (Some(a), Some(b)) = (&self.ref_a, &self.ref_b) {
            cmd = cmd.arg(format!("{a}..{b}"));
        }

        cmd
    }

    /// Build the raw diff command (with optional --stat).
    pub(crate) fn build_raw_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("git").arg("-C").arg(self.repo_path.to_string_lossy().as_ref()).arg("diff");

        if self.cached {
            cmd = cmd.arg("--cached");
        }

        if self.stat {
            cmd = cmd.arg("--stat");
        }

        if let (Some(a), Some(b)) = (&self.ref_a, &self.ref_b) {
            cmd = cmd.arg(format!("{a}..{b}"));
        }

        cmd
    }

    pub(crate) fn ref_range(&self) -> Option<String> {
        match (&self.ref_a, &self.ref_b) {
            (Some(a), Some(b)) => Some(format!("{a}..{b}")),
            _ => None,
        }
    }
}

/// Parse numstat output into file-level stats, then combine with raw diff output.
pub(crate) fn parse_numstat_output(output: &Output) -> Result<Vec<DiffFile>, DiffError> {
    let stdout = stdout_string(output);
    let mut files = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }

        let insertions = parts[0].parse::<usize>().unwrap_or(0);
        let deletions = parts[1].parse::<usize>().unwrap_or(0);
        let path = parts[2].to_string();

        // Determine status based on insertions/deletions
        let status = FileStatus::Modified;

        // Handle renames (numstat shows "old => new" in some cases)
        let (file_path, _old_path) = if path.contains(" => ") {
            let parts: Vec<&str> = path.splitn(2, " => ").collect();
            (parts[1].to_string(), Some(parts[0].to_string()))
        } else {
            (path, None)
        };

        files.push(DiffFile { path: file_path, status, insertions, deletions });
    }

    Ok(files)
}

pub(crate) fn check_diff_errors(output: &Output, ref_range: Option<&str>) -> Result<(), DiffError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    let code = output.status.code().unwrap_or(-1);

    if stderr.contains("unknown revision") || stderr.contains("bad revision") {
        let reference = ref_range.unwrap_or("unknown").to_string();
        return Err(DiffError::RefNotFound { reference });
    }

    if stderr.contains("invalid range") || stderr.contains("bad range") {
        let range = ref_range.unwrap_or("unknown").to_string();
        return Err(DiffError::InvalidRange { range });
    }

    Err(DiffError::Command(CommandError::Failed {
        args: "diff".to_string(),
        code,
        stdout: stdout_string(output),
        stderr,
    }))
}

pub(crate) fn build_diff_result(files: Vec<DiffFile>, raw_output: &Output) -> DiffResult {
    let raw = stdout_string(raw_output);

    let stats = DiffStats {
        files_changed: files.len(),
        insertions: files.iter().map(|f| f.insertions).sum(),
        deletions: files.iter().map(|f| f.deletions).sum(),
    };

    DiffResult { files, stats, raw }
}
