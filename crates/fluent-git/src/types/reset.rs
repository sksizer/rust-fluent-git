#[derive(Debug, Clone, PartialEq)]
pub enum ResetMode {
    Soft,
    Mixed,
    Hard,
}

#[derive(Debug, Clone)]
pub struct ResetResult {
    pub sha: String,
    pub mode: ResetMode,
}
