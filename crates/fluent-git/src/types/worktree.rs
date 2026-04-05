use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: String,
    pub sha: String,
    pub is_main: bool,
    pub locked: bool,
}

#[derive(Debug, Clone)]
pub struct WorktreeAddResult {
    pub path: PathBuf,
    pub branch: String,
    pub sha: String,
    pub created_branch: bool,
}

#[derive(Debug, Clone)]
pub struct WorktreeRemoveResult {
    pub path: PathBuf,
    pub branch: String,
    pub pruned: bool,
}

#[derive(Debug, Clone)]
pub struct WorktreeListResult {
    pub worktrees: Vec<WorktreeInfo>,
    pub main: WorktreeInfo,
}

impl WorktreeListResult {
    pub fn linked(&self) -> Vec<&WorktreeInfo> {
        self.worktrees.iter().filter(|w| !w.is_main).collect()
    }

    pub fn find_by_branch(&self, branch: &str) -> Option<&WorktreeInfo> {
        self.worktrees.iter().find(|w| w.branch == branch)
    }

    pub fn find_by_path(&self, path: &PathBuf) -> Option<&WorktreeInfo> {
        self.worktrees.iter().find(|w| w.path == *path)
    }
}

#[derive(Debug, Clone)]
pub struct WorktreeMoveResult {
    pub old_path: PathBuf,
    pub new_path: PathBuf,
    pub branch: String,
}

#[derive(Debug, Clone)]
pub struct WorktreeLockResult {
    pub path: PathBuf,
    pub reason: Option<String>,
    pub was_already_locked: bool,
}

#[derive(Debug, Clone)]
pub struct WorktreePruneResult {
    pub pruned: Vec<PathBuf>,
}
