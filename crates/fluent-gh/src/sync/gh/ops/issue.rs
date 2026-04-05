use crate::error::IssueError;
use crate::ops::issue::{
    parse_close_output, parse_comment_output, parse_create_output, parse_list_output, parse_view_output,
};
use crate::ops::{IssueCloseBuilder, IssueCommentBuilder, IssueCreateBuilder, IssueListBuilder, IssueViewBuilder};
use crate::types::{IssueCreateResult, IssueInfo};

#[cfg(not(feature = "tokio"))]
impl<'a> IssueCreateBuilder<'a> {
    pub fn run(self) -> Result<IssueCreateResult, IssueError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> IssueListBuilder<'a> {
    pub fn run(self) -> Result<Vec<IssueInfo>, IssueError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> IssueViewBuilder<'a> {
    pub fn run(self) -> Result<IssueInfo, IssueError> {
        let number = self.number();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_view_output(&output, number)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> IssueCloseBuilder<'a> {
    pub fn run(self) -> Result<(), IssueError> {
        let number = self.number();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_close_output(&output, number)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> IssueCommentBuilder<'a> {
    pub fn run(self) -> Result<(), IssueError> {
        let number = self.number();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_comment_output(&output, number)
    }
}
