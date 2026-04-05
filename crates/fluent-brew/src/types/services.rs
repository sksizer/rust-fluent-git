use serde::{Deserialize, Serialize};

/// Service info from `brew services list --json` or `brew services info --json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub status: String,
    pub user: Option<String>,
    pub file: Option<String>,
    pub exit_code: Option<i32>,
}
