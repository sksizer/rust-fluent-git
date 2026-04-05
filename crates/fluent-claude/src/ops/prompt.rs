//! Builder for `claude -p` prompt operations.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::PromptError;
use crate::types::{Effort, OutputFormat, PermissionMode, PromptConfig, PromptResult};
use fluent_core::{stderr_string, stdout_string};

// ── PromptBuilder ────────────────────────────────────────────────────

pub struct PromptBuilder {
    prompt: String,
    model: Option<String>,
    effort: Option<Effort>,
    system_prompt: Option<String>,
    append_system_prompt: Option<String>,
    output_format: Option<OutputFormat>,
    json_schema: Option<String>,
    max_budget_usd: Option<f64>,
    permission_mode: Option<PermissionMode>,
    allowed_tools: Vec<String>,
    disallowed_tools: Vec<String>,
    tools: Option<Vec<String>>,
    add_dirs: Vec<PathBuf>,
    working_dir: Option<PathBuf>,
    mcp_configs: Vec<String>,
    session_id: Option<String>,
    name: Option<String>,
    bare: bool,
    fallback_model: Option<String>,
    no_session_persistence: bool,
}

impl PromptBuilder {
    pub(crate) fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            model: None,
            effort: None,
            system_prompt: None,
            append_system_prompt: None,
            output_format: None,
            json_schema: None,
            max_budget_usd: None,
            permission_mode: None,
            allowed_tools: Vec::new(),
            disallowed_tools: Vec::new(),
            tools: None,
            add_dirs: Vec::new(),
            working_dir: None,
            mcp_configs: Vec::new(),
            session_id: None,
            name: None,
            bare: false,
            fallback_model: None,
            no_session_persistence: false,
        }
    }

    /// Set the model to use.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the effort level.
    pub fn effort(mut self, effort: Effort) -> Self {
        self.effort = Some(effort);
        self
    }

    /// Set a system prompt.
    pub fn system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Append to the system prompt.
    pub fn append_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.append_system_prompt = Some(prompt.into());
        self
    }

    /// Set the output format.
    pub fn output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = Some(format);
        self
    }

    /// Set a JSON schema for structured output.
    pub fn json_schema(mut self, schema: impl Into<String>) -> Self {
        self.json_schema = Some(schema.into());
        self
    }

    /// Set the maximum budget in USD.
    pub fn max_budget_usd(mut self, budget: f64) -> Self {
        self.max_budget_usd = Some(budget);
        self
    }

    /// Set the permission mode.
    pub fn permission_mode(mut self, mode: PermissionMode) -> Self {
        self.permission_mode = Some(mode);
        self
    }

    /// Add an allowed tool (can be called multiple times).
    pub fn allowed_tool(mut self, tool: impl Into<String>) -> Self {
        self.allowed_tools.push(tool.into());
        self
    }

    /// Add a disallowed tool (can be called multiple times).
    pub fn disallowed_tool(mut self, tool: impl Into<String>) -> Self {
        self.disallowed_tools.push(tool.into());
        self
    }

    /// Set the list of tools to use.
    pub fn tools(mut self, tools: Vec<String>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Add a directory to include (can be called multiple times).
    pub fn add_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.add_dirs.push(dir.into());
        self
    }

    /// Set the working directory for the command.
    pub fn working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Add an MCP config (can be called multiple times).
    pub fn mcp_config(mut self, config: impl Into<String>) -> Self {
        self.mcp_configs.push(config.into());
        self
    }

    /// Resume a previous session by ID.
    pub fn session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = Some(id.into());
        self
    }

    /// Set a name for this session.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Run in bare mode (no rich formatting).
    pub fn bare(mut self) -> Self {
        self.bare = true;
        self
    }

    /// Set a fallback model.
    pub fn fallback_model(mut self, model: impl Into<String>) -> Self {
        self.fallback_model = Some(model.into());
        self
    }

    /// Disable session persistence.
    pub fn no_session_persistence(mut self) -> Self {
        self.no_session_persistence = true;
        self
    }

    /// Snapshot the current builder state into a serializable config.
    pub fn config(&self) -> PromptConfig {
        PromptConfig {
            prompt: self.prompt.clone(),
            model: self.model.clone(),
            effort: self.effort,
            system_prompt: self.system_prompt.clone(),
            append_system_prompt: self.append_system_prompt.clone(),
            output_format: self.output_format,
            json_schema: self.json_schema.clone(),
            max_budget_usd: self.max_budget_usd,
            permission_mode: self.permission_mode,
            allowed_tools: self.allowed_tools.clone(),
            disallowed_tools: self.disallowed_tools.clone(),
            tools: self.tools.clone(),
            add_dirs: self.add_dirs.clone(),
            working_dir: self.working_dir.clone(),
            mcp_configs: self.mcp_configs.clone(),
            session_id: self.session_id.clone(),
            name: self.name.clone(),
            bare: self.bare,
            fallback_model: self.fallback_model.clone(),
            no_session_persistence: self.no_session_persistence,
        }
    }

    /// Reconstruct a builder from a serialized config.
    pub fn from_config(config: PromptConfig) -> Self {
        Self {
            prompt: config.prompt,
            model: config.model,
            effort: config.effort,
            system_prompt: config.system_prompt,
            append_system_prompt: config.append_system_prompt,
            output_format: config.output_format,
            json_schema: config.json_schema,
            max_budget_usd: config.max_budget_usd,
            permission_mode: config.permission_mode,
            allowed_tools: config.allowed_tools,
            disallowed_tools: config.disallowed_tools,
            tools: config.tools,
            add_dirs: config.add_dirs,
            working_dir: config.working_dir,
            mcp_configs: config.mcp_configs,
            session_id: config.session_id,
            name: config.name,
            bare: config.bare,
            fallback_model: config.fallback_model,
            no_session_persistence: config.no_session_persistence,
        }
    }

    /// Expose the working directory for sync/async execution layers.
    pub(crate) fn get_working_dir(&self) -> Option<&Path> {
        self.working_dir.as_deref()
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("claude").arg("-p");

        // Default to JSON output unless explicitly overridden
        let format = self.output_format.unwrap_or(OutputFormat::Json);
        cmd = cmd.arg("--output-format").arg(format.to_string());

        if let Some(ref model) = self.model {
            cmd = cmd.arg("--model").arg(model);
        }

        if let Some(effort) = self.effort {
            cmd = cmd.arg("--effort").arg(effort.to_string());
        }

        if let Some(ref system_prompt) = self.system_prompt {
            cmd = cmd.arg("--system-prompt").arg(system_prompt);
        }

        if let Some(ref append) = self.append_system_prompt {
            cmd = cmd.arg("--append-system-prompt").arg(append);
        }

        if let Some(ref schema) = self.json_schema {
            cmd = cmd.arg("--json-schema").arg(schema);
        }

        if let Some(budget) = self.max_budget_usd {
            cmd = cmd.arg("--max-budget-usd").arg(budget.to_string());
        }

        if let Some(mode) = self.permission_mode {
            cmd = cmd.arg("--permission-mode").arg(mode.to_string());
        }

        for tool in &self.allowed_tools {
            cmd = cmd.arg("--allowedTools").arg(tool);
        }

        for tool in &self.disallowed_tools {
            cmd = cmd.arg("--disallowedTools").arg(tool);
        }

        if let Some(ref tools) = self.tools {
            for tool in tools {
                cmd = cmd.arg("--tool").arg(tool);
            }
        }

        for dir in &self.add_dirs {
            cmd = cmd.arg("--add-dir").arg(dir.to_string_lossy().as_ref());
        }

        for config in &self.mcp_configs {
            cmd = cmd.arg("--mcp-config").arg(config);
        }

        if let Some(ref session_id) = self.session_id {
            cmd = cmd.arg("--session-id").arg(session_id);
        }

        if let Some(ref name) = self.name {
            cmd = cmd.arg("--name").arg(name);
        }

        if self.bare {
            cmd = cmd.arg("--bare");
        }

        if let Some(ref fallback) = self.fallback_model {
            cmd = cmd.arg("--fallback-model").arg(fallback);
        }

        if self.no_session_persistence {
            cmd = cmd.arg("--no-session-persistence");
        }

        // TODO: working_dir is not passed as a CLI flag — it needs to be
        // handled at the execution layer (e.g., std::process::Command::current_dir).
        // ShellCommand from cmd-spec may not support setting CWD directly.

        // Prompt is the last positional argument
        cmd = cmd.arg(&self.prompt);

        cmd
    }
}

// ── Output Parsing ───────────────────────────────────────────────────

pub(crate) fn parse_prompt_output(output: &Output) -> Result<PromptResult, PromptError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let result: PromptResult = serde_json::from_str(&stdout)?;
        if result.is_error {
            return Err(PromptError::ExecutionFailed { reason: result.result });
        }
        return Ok(result);
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") || lower.contains("not authenticated") {
        return Err(PromptError::NotAuthenticated);
    }

    if lower.contains("model") && (lower.contains("not available") || lower.contains("not found")) {
        return Err(PromptError::ModelNotAvailable { model: String::new() });
    }

    if lower.contains("budget") && lower.contains("exceeded") {
        return Err(PromptError::BudgetExceeded { budget_usd: 0.0 });
    }

    if lower.contains("permission") && lower.contains("denied") {
        return Err(PromptError::PermissionDenied { tool: String::new() });
    }

    if lower.contains("schema") && lower.contains("validation") {
        return Err(PromptError::SchemaValidationFailed { reason: stderr.clone() });
    }

    Err(PromptError::Command(fluent_core::CommandError::Failed {
        args: "claude -p".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
