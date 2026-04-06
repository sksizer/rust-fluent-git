use crate::error::WorkflowError;
use crate::ops::workflow::{
    parse_disable_output, parse_enable_output, parse_list_output, parse_run_output, parse_view_output,
};
use crate::ops::{
    WorkflowDisableBuilder, WorkflowEnableBuilder, WorkflowListBuilder, WorkflowRunBuilder, WorkflowViewBuilder,
};
use crate::types::WorkflowInfo;

#[cfg(feature = "tokio")]
impl<'a> WorkflowListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<WorkflowInfo>, WorkflowError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> WorkflowViewBuilder<'a> {
    pub async fn run_async(self) -> Result<WorkflowInfo, WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_view_output(&output, &id, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> WorkflowRunBuilder<'a> {
    pub async fn run_async(self) -> Result<(), WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_run_output(&output, &id, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> WorkflowEnableBuilder<'a> {
    pub async fn run_async(self) -> Result<(), WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_enable_output(&output, &id, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> WorkflowDisableBuilder<'a> {
    pub async fn run_async(self) -> Result<(), WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_disable_output(&output, &id, &slug)
    }
}
