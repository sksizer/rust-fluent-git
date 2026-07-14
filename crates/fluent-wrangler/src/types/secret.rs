use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub secret_type: Option<String>,
}
