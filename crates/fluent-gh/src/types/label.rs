use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelInfo {
    pub name: String,
    pub color: String,
    pub description: String,
}
