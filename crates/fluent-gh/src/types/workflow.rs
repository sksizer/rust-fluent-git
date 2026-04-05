use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowInfo {
    pub id: u64,
    pub name: String,
    pub state: String,
    pub path: String,
}
