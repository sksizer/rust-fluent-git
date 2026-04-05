use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config key '{key}' not found")]
    KeyNotFound { key: String },
    #[error("invalid config key '{key}': {reason}")]
    InvalidKey { key: String, reason: String },
    #[error("invalid config value for '{key}': expected {expected}, got '{value}'")]
    InvalidValue { key: String, value: String, expected: String },
    #[error("config file is locked by another process")]
    Locked,
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
