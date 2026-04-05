use serde::{Deserialize, Serialize};

/// Information about a Lima VM instance, as returned by `limactl list --json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
    pub name: String,
    pub hostname: Option<String>,
    pub status: String,
    pub dir: String,
    pub vm_type: String,
    pub arch: String,
    pub cpus: u32,
    pub memory: u64,
    pub disk: u64,
    pub ssh_local_port: Option<u16>,
    pub ssh_address: Option<String>,
    pub ssh_config_file: Option<String>,
    pub protected: bool,
}
