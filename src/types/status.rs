use super::FileChange;

#[derive(Debug, Clone)]
pub struct StatusResult {
    pub branch: String,
    pub staged: Vec<FileChange>,
    pub modified: Vec<FileChange>,
    pub untracked: Vec<String>,
    pub ahead: usize,
    pub behind: usize,
}

impl StatusResult {
    pub fn is_clean(&self) -> bool {
        self.staged.is_empty() && self.modified.is_empty() && self.untracked.is_empty()
    }

    pub fn modified(&self) -> Vec<String> {
        self.modified.iter().map(|f| f.path.clone()).collect()
    }
}
