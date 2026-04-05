use crate::error::RunError;
use crate::ops::run::{parse_list_output, parse_rerun_output, parse_view_output, parse_watch_output};
use crate::ops::{RunListBuilder, RunRerunBuilder, RunViewBuilder, RunWatchBuilder};
use crate::types::{RunInfo, RunRerunResult};

#[cfg(not(feature = "tokio"))]
impl<'a> RunListBuilder<'a> {
    pub fn run(self) -> Result<Vec<RunInfo>, RunError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> RunViewBuilder<'a> {
    pub fn run(self) -> Result<RunInfo, RunError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_view_output(&output, &id, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> RunRerunBuilder<'a> {
    pub fn run(self) -> Result<RunRerunResult, RunError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_rerun_output(&output, &id, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> RunWatchBuilder<'a> {
    pub fn run(self) -> Result<(), RunError> {
        let id = self.id().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_watch_output(&output, &id, &slug)
    }
}
