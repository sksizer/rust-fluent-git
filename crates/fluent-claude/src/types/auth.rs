use serde::{Deserialize, Serialize};

/// Authentication status for the Claude Code CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatus {
    pub logged_in: bool,
    pub account: Option<String>,
}
