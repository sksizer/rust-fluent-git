use serde::Deserialize;

use crate::types::common::{GhLabel, GhUser};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrInfo {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: GhUser,
    pub url: String,
    pub head_ref_name: String,
    pub base_ref_name: String,
    pub is_draft: bool,
    pub created_at: String,
    pub labels: Vec<GhLabel>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrCreateResult {
    pub number: u64,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrMergeResult {
    pub number: u64,
    pub title: String,
    pub state: String,
}
