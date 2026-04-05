use serde::{Deserialize, Serialize};

/// Response from `brew info --json=v2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoResponse {
    #[serde(default)]
    pub formulae: Vec<FormulaInfo>,
    #[serde(default)]
    pub casks: Vec<CaskInfo>,
}

/// Formula info from `brew info --json=v2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaInfo {
    pub name: String,
    pub full_name: String,
    pub tap: String,
    #[serde(default)]
    pub oldnames: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub desc: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub versions: FormulaVersions,
    pub revision: u32,
    pub keg_only: bool,
    #[serde(default)]
    pub installed: Vec<InstalledVersion>,
    pub linked_keg: Option<String>,
    pub pinned: bool,
    pub outdated: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaVersions {
    pub stable: Option<String>,
    pub head: Option<String>,
    pub bottle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledVersion {
    pub version: String,
    #[serde(default)]
    pub installed_as_dependency: bool,
    #[serde(default)]
    pub installed_on_request: bool,
}

/// Cask info from `brew info --json=v2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaskInfo {
    pub token: String,
    pub full_token: String,
    pub tap: Option<String>,
    pub name: Vec<String>,
    pub desc: Option<String>,
    pub homepage: Option<String>,
    pub version: Option<String>,
    pub installed: Option<String>,
    pub outdated: bool,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Response from `brew outdated --json=v2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdatedResponse {
    #[serde(default)]
    pub formulae: Vec<OutdatedFormula>,
    #[serde(default)]
    pub casks: Vec<OutdatedCask>,
}

/// An outdated formula from `brew outdated --json=v2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdatedFormula {
    pub name: String,
    pub installed_versions: Vec<String>,
    pub current_version: String,
    pub pinned: bool,
    pub pinned_version: Option<String>,
}

/// An outdated cask from `brew outdated --json=v2`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdatedCask {
    pub name: String,
    pub installed_versions: String,
    pub current_version: String,
}
