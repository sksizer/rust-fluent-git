use serde::{Deserialize, Serialize};

/// System-level information returned by `limactl info`.
///
/// Lima's info output is complex and variable across versions, so we capture
/// the version explicitly and flatten everything else into a generic value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}
