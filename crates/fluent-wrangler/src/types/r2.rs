use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2BucketInfo {
    pub name: String,
    pub creation_date: Option<String>,
    pub location: Option<String>,
    pub storage_class: Option<String>,
}
