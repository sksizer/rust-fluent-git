use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ── Response Types ───────────────────────────────────────────────────

/// Parsed result from a `claude -p` invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResult {
    pub result: String,
    pub session_id: String,
    pub is_error: bool,
    pub duration_ms: u64,
    pub duration_api_ms: u64,
    pub num_turns: u32,
    pub stop_reason: String,
    pub total_cost_usd: f64,
    pub usage: Usage,
    pub terminal_reason: String,
    pub uuid: String,
}

/// Token usage breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

// ── Config Snapshot ──────────────────────────────────────────────────

/// Serializable snapshot of builder state for workflow checkpointing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    pub prompt: String,
    pub model: Option<String>,
    pub effort: Option<Effort>,
    pub system_prompt: Option<String>,
    pub append_system_prompt: Option<String>,
    pub output_format: Option<OutputFormat>,
    pub json_schema: Option<String>,
    pub max_budget_usd: Option<f64>,
    pub permission_mode: Option<PermissionMode>,
    pub allowed_tools: Vec<String>,
    pub disallowed_tools: Vec<String>,
    pub tools: Option<Vec<String>>,
    pub add_dirs: Vec<PathBuf>,
    pub working_dir: Option<PathBuf>,
    pub mcp_configs: Vec<String>,
    pub session_id: Option<String>,
    pub name: Option<String>,
    pub bare: bool,
    pub fallback_model: Option<String>,
    pub no_session_persistence: bool,
}

// ── Enums ────────────────────────────────────────────────────────────

/// Effort level for the model.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effort {
    Low,
    Medium,
    High,
    Max,
}

impl fmt::Display for Effort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Effort::Low => write!(f, "low"),
            Effort::Medium => write!(f, "medium"),
            Effort::High => write!(f, "high"),
            Effort::Max => write!(f, "max"),
        }
    }
}

/// Output format for the CLI.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Text,
    Json,
    StreamJson,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::StreamJson => write!(f, "stream-json"),
        }
    }
}

/// Permission mode for the CLI.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionMode {
    Default,
    Auto,
    Plan,
    AcceptEdits,
    BypassPermissions,
    DontAsk,
}

impl fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionMode::Default => write!(f, "default"),
            PermissionMode::Auto => write!(f, "auto"),
            PermissionMode::Plan => write!(f, "plan"),
            PermissionMode::AcceptEdits => write!(f, "acceptEdits"),
            PermissionMode::BypassPermissions => write!(f, "bypassPermissions"),
            PermissionMode::DontAsk => write!(f, "dontAsk"),
        }
    }
}
