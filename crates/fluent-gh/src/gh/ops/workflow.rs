use crate::error::WorkflowError;
use crate::ops::workflow::{
    parse_disable_output, parse_enable_output, parse_list_output, parse_run_output, parse_view_output,
};
use crate::ops::{
    WorkflowDisableBuilder, WorkflowEnableBuilder, WorkflowListBuilder, WorkflowRunBuilder, WorkflowViewBuilder,
};
use crate::types::WorkflowInfo;

#[cfg(not(feature = "blocking"))]
impl<'a> WorkflowListBuilder<'a> {
    pub async fn run(self) -> Result<Vec<WorkflowInfo>, WorkflowError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> WorkflowViewBuilder<'a> {
    pub async fn run(self) -> Result<WorkflowInfo, WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_view_output(&output, &id, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> WorkflowRunBuilder<'a> {
    pub async fn run(self) -> Result<(), WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_run_output(&output, &id, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> WorkflowEnableBuilder<'a> {
    pub async fn run(self) -> Result<(), WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_enable_output(&output, &id, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> WorkflowDisableBuilder<'a> {
    pub async fn run(self) -> Result<(), WorkflowError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_disable_output(&output, &id, &slug)
    }
}
