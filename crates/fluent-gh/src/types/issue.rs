use serde::Deserialize;

use crate::types::common::{GhLabel, GhUser};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueInfo {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: GhUser,
    pub url: String,
    pub created_at: String,
    pub labels: Vec<GhLabel>,
    pub assignees: Vec<GhUser>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCreateResult {
    pub number: u64,
    pub url: String,
    pub title: String,
}
