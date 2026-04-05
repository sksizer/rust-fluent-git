#[derive(Debug, Clone)]
pub struct CleanResult {
    pub removed_files: Vec<String>,
    pub removed_dirs: Vec<String>,
}
