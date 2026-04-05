use super::Author;

#[derive(Debug, Clone)]
pub struct CommitResult {
    pub sha: String,
    pub short_sha: String,
    pub message: String,
    pub branch: String,
    pub author: Author,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}
