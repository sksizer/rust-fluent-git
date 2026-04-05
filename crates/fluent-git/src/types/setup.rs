use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub version: String,
    pub path: PathBuf,
}
