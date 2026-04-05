use std::path::{Path, PathBuf};

/// Handle to a git repository. Proof that setup succeeded.
/// All repo operations hang off this type.
#[derive(Debug, Clone)]
pub struct Repo {
    path: PathBuf,
}

impl Repo {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
