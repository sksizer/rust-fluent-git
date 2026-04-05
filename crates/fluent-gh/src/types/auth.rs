use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthStatus {
    pub user: String,
    pub host: String,
    pub token_valid: bool,
}
