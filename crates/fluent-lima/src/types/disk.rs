use serde::{Deserialize, Serialize};

/// Information about a Lima disk, as returned by `limactl disk ls --json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub size: u64,
    pub dir: String,
    pub instance: String,
    #[serde(rename = "instanceDir")]
    pub instance_dir: String,
}
