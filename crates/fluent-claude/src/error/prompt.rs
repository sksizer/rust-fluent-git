use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    #[error("not authenticated; run `claude login`")]
    NotAuthenticated,
    #[error("model not available: {model}")]
    ModelNotAvailable { model: String },
    #[error("budget exceeded: ${budget_usd:.2}")]
    BudgetExceeded { budget_usd: f64 },
    #[error("permission denied for tool: {tool}")]
    PermissionDenied { tool: String },
    #[error("schema validation failed: {reason}")]
    SchemaValidationFailed { reason: String },
    #[error("execution failed: {reason}")]
    ExecutionFailed { reason: String },
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse claude output: {0}")]
    Parse(#[from] serde_json::Error),
}

impl From<CommandError> for PromptError {
    fn from(e: CommandError) -> Self {
        PromptError::Command(e)
    }
}
