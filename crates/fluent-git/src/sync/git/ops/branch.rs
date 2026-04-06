use crate::error::BranchError;
use crate::ops::branch::{
    build_current_command, parse_create_output, parse_current_output, parse_delete_output, parse_list_output,
    parse_rename_output,
};
use crate::ops::{BranchBuilder, BranchCreateBuilder, BranchDeleteBuilder, BranchListBuilder, BranchRenameBuilder};
use crate::types::BranchInfo;

#[cfg(feature = "blocking")]
impl<'a> BranchBuilder<'a> {
    /// Get the name of the current branch.
    pub fn current(&self) -> Result<String, BranchError> {
        let cmd = build_current_command(self.repo_path());
        let output = fluent_core::run_sync(&cmd)?;
        parse_current_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl<'a> BranchCreateBuilder<'a> {
    pub fn run(self) -> Result<(), BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, self.name())
    }
}

#[cfg(feature = "blocking")]
impl<'a> BranchDeleteBuilder<'a> {
    pub fn run(self) -> Result<(), BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, self.name())
    }
}

#[cfg(feature = "blocking")]
impl<'a> BranchListBuilder<'a> {
    pub fn run(self) -> Result<Vec<BranchInfo>, BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl<'a> BranchRenameBuilder<'a> {
    pub fn run(self) -> Result<(), BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_rename_output(&output, self.old_name())
    }
}
