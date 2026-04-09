use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvNamespaceInfo {
    pub id: String,
    pub title: String,
    pub supports_url_encoding: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvKeyInfo {
    pub name: String,
    pub expiration: Option<u64>,
    pub metadata: Option<serde_json::Value>,
}
