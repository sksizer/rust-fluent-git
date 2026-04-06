use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoAmIInfo {
    pub account_id: Option<String>,
    pub account_name: Option<String>,
}
