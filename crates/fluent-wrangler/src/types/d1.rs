use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1DatabaseInfo {
    pub uuid: String,
    pub name: String,
    pub created_at: Option<String>,
    pub version: Option<String>,
    pub num_tables: Option<u64>,
    pub file_size: Option<u64>,
    pub running_in_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1ExecuteResult {
    pub results: Option<Vec<serde_json::Value>>,
    pub success: bool,
    pub meta: Option<D1ExecuteMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D1ExecuteMeta {
    pub changes: Option<u64>,
    pub duration: Option<f64>,
    pub rows_read: Option<u64>,
    pub rows_written: Option<u64>,
}
