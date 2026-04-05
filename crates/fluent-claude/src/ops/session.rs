//! Builder for `claude` session resume/continue operations.

use std::path::PathBuf;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::SessionError;
use crate::types::PromptResult;
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── SessionMode ──────────────────────────────────────────────────────

/// Whether to resume a specific session or continue the most recent one.
#[derive(Debug, Clone)]
pub enum SessionMode {
    /// Resume a specific session by ID.
    Resume { session_id: String },
    /// Continue the most recent session in the working directory.
    Continue,
}

// ── SessionResumeBuilder ─────────────────────────────────────────────

/// Builder for resuming or continuing a Claude Code session.
///
/// No lifetime parameter — all data is owned.
pub struct SessionResumeBuilder {
    mode: SessionMode,
    prompt: Option<String>,
    fork_session: bool,
    working_dir: Option<PathBuf>,
    model: Option<String>,
    output_format: String,
}

impl SessionResumeBuilder {
    /// Create a builder that resumes a specific session by ID.
    pub fn new_resume(session_id: impl Into<String>) -> Self {
        Self {
            mode: SessionMode::Resume { session_id: session_id.into() },
            prompt: None,
            fork_session: false,
            working_dir: None,
            model: None,
            output_format: "json".to_string(),
        }
    }

    /// Create a builder that continues the most recent session.
    pub fn new_continue() -> Self {
        Self {
            mode: SessionMode::Continue,
            prompt: None,
            fork_session: false,
            working_dir: None,
            model: None,
            output_format: "json".to_string(),
        }
    }

    /// Set an optional follow-up prompt for the session.
    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Fork the session instead of continuing it in-place.
    pub fn fork_session(mut self) -> Self {
        self.fork_session = true;
        self
    }

    /// Set the working directory for the session.
    pub fn working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Set the model to use.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the output format (defaults to `"json"`).
    pub fn output_format(mut self, format: impl Into<String>) -> Self {
        self.output_format = format.into();
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("claude").arg("-p");

        match &self.mode {
            SessionMode::Resume { session_id } => {
                cmd = cmd.arg("--resume").arg(session_id);
            }
            SessionMode::Continue => {
                cmd = cmd.arg("--continue");
            }
        }

        cmd = cmd.arg("--output-format").arg(&self.output_format);

        if self.fork_session {
            cmd = cmd.arg("--fork");
        }

        if let Some(ref dir) = self.working_dir {
            cmd = cmd.arg("--cwd").arg(dir.to_string_lossy().as_ref());
        }

        if let Some(ref model) = self.model {
            cmd = cmd.arg("--model").arg(model);
        }

        if let Some(ref prompt) = self.prompt {
            cmd = cmd.arg(prompt);
        }

        cmd
    }
}

pub(crate) fn parse_resume_output(output: &Output) -> Result<PromptResult, SessionError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let result: PromptResult = serde_json::from_str(&stdout)?;
        return Ok(result);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("not found") || lower.contains("no session") {
        // Try to extract a session id from the error for a better message.
        return Err(SessionError::NotFound { session_id: String::new() });
    }

    if lower.contains("auth") || lower.contains("login") || lower.contains("not authenticated") {
        return Err(SessionError::NotAuthenticated);
    }

    Err(SessionError::Command(CommandError::Failed {
        args: "claude -p --resume/--continue".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
