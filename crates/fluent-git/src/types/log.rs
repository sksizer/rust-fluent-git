use super::Author;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub sha: String,
    pub short_sha: String,
    pub message: String,
    pub author: Author,
    pub timestamp: String,
    pub refs: Vec<String>,
}
