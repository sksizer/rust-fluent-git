use crate::ops::{AuthStatusBuilder, PromptBuilder, SessionResumeBuilder};

/// Entry point for Claude Code CLI operations.
///
/// Stateless — configuration is per-invocation.
#[derive(Debug, Clone, Default)]
pub struct ClaudeCode;

impl ClaudeCode {
    pub fn new() -> Self {
        Self
    }

    /// Start building a prompt invocation.
    pub fn prompt(&self, prompt: impl Into<String>) -> PromptBuilder {
        PromptBuilder::new(prompt)
    }

    /// Resume a previous session by ID.
    pub fn resume(&self, session_id: impl Into<String>) -> SessionResumeBuilder {
        SessionResumeBuilder::new_resume(session_id)
    }

    /// Continue the most recent session in the working directory.
    pub fn continue_last(&self) -> SessionResumeBuilder {
        SessionResumeBuilder::new_continue()
    }

    /// Check authentication status.
    pub fn auth_status(&self) -> AuthStatusBuilder {
        AuthStatusBuilder::new()
    }
}

impl fluent_core::tool::CliTool for ClaudeCode {
    fn program() -> &'static str {
        "claude"
    }
}
