use serde::Deserialize;

use crate::types::common::GhUser;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoInfo {
    pub name: String,
    pub owner: GhUser,
    pub description: Option<String>,
    pub url: String,
    pub is_private: bool,
    pub is_fork: bool,
    pub default_branch_ref: RepoDefaultBranch,
    pub stargazer_count: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoDefaultBranch {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoCreateResult {
    pub url: String,
    pub name: String,
}
