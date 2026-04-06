use crate::error::BranchError;
use crate::ops::branch::{
    build_current_command, parse_create_output, parse_current_output, parse_delete_output, parse_list_output,
    parse_rename_output,
};
use crate::ops::{BranchBuilder, BranchCreateBuilder, BranchDeleteBuilder, BranchListBuilder, BranchRenameBuilder};
use crate::types::BranchInfo;

#[cfg(feature = "tokio")]
impl<'a> BranchBuilder<'a> {
    /// Get the name of the current branch (async).
    pub async fn current_async(&self) -> Result<String, BranchError> {
        let cmd = build_current_command(self.repo_path());
        let output = fluent_core::run_async(&cmd).await?;
        parse_current_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl<'a> BranchCreateBuilder<'a> {
    pub async fn run_async(self) -> Result<(), BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, self.name())
    }
}

#[cfg(feature = "tokio")]
impl<'a> BranchDeleteBuilder<'a> {
    pub async fn run_async(self) -> Result<(), BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, self.name())
    }
}

#[cfg(feature = "tokio")]
impl<'a> BranchListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<BranchInfo>, BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl<'a> BranchRenameBuilder<'a> {
    pub async fn run_async(self) -> Result<(), BranchError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_rename_output(&output, self.old_name())
    }
}
