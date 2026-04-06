use crate::error::CommitError;
use crate::ops::CommitBuilder;
use crate::ops::commit::{parse_commit_details, parse_commit_output};
use crate::types::CommitResult;

#[cfg(feature = "tokio")]
impl<'a> CommitBuilder<'a> {
    pub async fn run_async(self) -> Result<CommitResult, CommitError> {
        let cmd = self.build_commit_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_commit_output(&output)?;

        let show_cmd = self.build_show_command();
        let show_output = fluent_core::run_async(&show_cmd).await?;
        parse_commit_details(&show_output)
    }
}
