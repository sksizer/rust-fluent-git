#[derive(Debug, Clone)]
pub struct MergeResult {
    pub sha: String,
    pub strategy: String,
    pub fast_forward: bool,
    pub files_changed: usize,
}
