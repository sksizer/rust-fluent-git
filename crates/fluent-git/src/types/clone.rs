use std::path::PathBuf;

use super::Repo;

#[derive(Debug, Clone)]
pub struct CloneResult {
    pub path: PathBuf,
    pub remote: String,
    pub branch: String,
    pub shallow: bool,
}

impl CloneResult {
    pub fn into_repo(self) -> Repo {
        Repo::new(self.path)
    }
}
