use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInfo {
    pub database_id: u64,
    pub display_title: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub head_branch: String,
    pub event: String,
    pub created_at: String,
    pub url: String,
    pub workflow_name: String,
}

#[derive(Debug, Clone)]
pub struct RunRerunResult;
