use std::path::PathBuf;

use super::Repo;

#[derive(Debug, Clone)]
pub struct InitResult {
    pub path: PathBuf,
    pub branch: String,
    pub bare: bool,
}

impl InitResult {
    pub fn into_repo(self) -> Repo {
        Repo::new(self.path)
    }
}
