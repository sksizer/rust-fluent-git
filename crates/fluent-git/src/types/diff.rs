use super::FileStatus;

#[derive(Debug, Clone)]
pub struct DiffResult {
    pub files: Vec<DiffFile>,
    pub stats: DiffStats,
    pub raw: String,
}

impl DiffResult {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct DiffFile {
    pub path: String,
    pub status: FileStatus,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone)]
pub struct DiffStats {
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}
