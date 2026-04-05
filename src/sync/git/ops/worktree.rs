use crate::error::WorktreeError;
use crate::ops::worktree::{
    build_list_command, find_branch_for_path, parse_add_output, parse_branch_check_output, parse_list_output,
    parse_lock_output, parse_move_output, parse_prune_dry_run_output, parse_prune_output, parse_remove_output,
    parse_rev_parse_output, parse_unlock_output,
};
use crate::ops::{
    WorktreeAddBuilder, WorktreeListBuilder, WorktreeLockBuilder, WorktreeMoveBuilder, WorktreePruneBuilder,
    WorktreeRemoveBuilder, WorktreeUnlockBuilder,
};
use crate::types::{
    WorktreeAddResult, WorktreeListResult, WorktreeLockResult, WorktreeMoveResult, WorktreePruneResult,
    WorktreeRemoveResult,
};

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreeAddBuilder<'a> {
    pub fn run(self) -> Result<WorktreeAddResult, WorktreeError> {
        // Check if branch already exists
        let check_cmd = self.build_branch_check_command();
        let check_output = crate::run::run_sync(&check_cmd)?;
        let branch_exists = parse_branch_check_output(&check_output);

        // Run the appropriate add command
        let (output, created_branch) = if branch_exists {
            let cmd = self.build_add_existing_command();
            (crate::run::run_sync(&cmd)?, false)
        } else {
            let cmd = self.build_add_new_branch_command();
            (crate::run::run_sync(&cmd)?, true)
        };

        parse_add_output(&output, self.path(), self.branch())?;

        // Get SHA from the new worktree
        let rev_cmd = self.build_rev_parse_command();
        let rev_output = crate::run::run_sync(&rev_cmd)?;
        let sha = parse_rev_parse_output(&rev_output);

        Ok(WorktreeAddResult {
            path: self.path().to_path_buf(),
            branch: self.branch().to_string(),
            sha,
            created_branch,
        })
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreeRemoveBuilder<'a> {
    pub fn run(self) -> Result<WorktreeRemoveResult, WorktreeError> {
        // Get branch info before removing
        let list_cmd = build_list_command(self.repo_path());
        let list_output = crate::run::run_sync(&list_cmd)?;
        let branch = find_branch_for_path(&list_output, self.path());

        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_remove_output(&output, self.path())?;

        Ok(WorktreeRemoveResult { path: self.path().to_path_buf(), branch, pruned: false })
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreeListBuilder<'a> {
    pub fn run(self) -> Result<WorktreeListResult, WorktreeError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreeMoveBuilder<'a> {
    pub fn run(self) -> Result<WorktreeMoveResult, WorktreeError> {
        // Get branch info before moving
        let list_cmd = build_list_command(self.repo_path());
        let list_output = crate::run::run_sync(&list_cmd)?;
        let branch = find_branch_for_path(&list_output, self.old_path());

        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_move_output(&output, self.old_path())?;

        Ok(WorktreeMoveResult {
            old_path: self.old_path().to_path_buf(),
            new_path: self.new_path().to_path_buf(),
            branch,
        })
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreeLockBuilder<'a> {
    pub fn run(self) -> Result<WorktreeLockResult, WorktreeError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_lock_output(&output, self.path(), self.reason_ref())
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreeUnlockBuilder<'a> {
    pub fn run(self) -> Result<(), WorktreeError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_unlock_output(&output, self.path())
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> WorktreePruneBuilder<'a> {
    pub fn run(self) -> Result<WorktreePruneResult, WorktreeError> {
        // First dry-run to see what would be pruned
        let dry_cmd = self.build_dry_run_command();
        let dry_output = crate::run::run_sync(&dry_cmd)?;
        let pruned = parse_prune_dry_run_output(&dry_output)?;

        // Then actually prune
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_prune_output(&output)?;

        Ok(WorktreePruneResult { pruned })
    }
}
