use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GhUser {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GhLabel {
    pub name: String,
}
