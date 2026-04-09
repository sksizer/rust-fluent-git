use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagesProjectInfo {
    pub name: String,
    pub subdomain: Option<String>,
    pub domains: Option<Vec<String>>,
    pub production_branch: Option<String>,
    pub created_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagesDeploymentInfo {
    pub id: String,
    pub environment: Option<String>,
    pub project_name: Option<String>,
    pub url: Option<String>,
    pub created_on: Option<String>,
}
