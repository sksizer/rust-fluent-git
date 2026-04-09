use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeIndexInfo {
    pub name: String,
    pub dimensions: Option<u32>,
    pub metric: Option<String>,
    pub description: Option<String>,
    pub created_on: Option<String>,
    pub modified_on: Option<String>,
}
