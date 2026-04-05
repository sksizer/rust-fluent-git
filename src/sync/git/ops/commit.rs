use crate::error::CommitError;
use crate::ops::commit::{parse_commit_details, parse_commit_output};
use crate::ops::CommitBuilder;
use crate::types::CommitResult;

#[cfg(not(feature = "tokio"))]
impl<'a> CommitBuilder<'a> {
    pub fn run(self) -> Result<CommitResult, CommitError> {
        let cmd = self.build_commit_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_commit_output(&output)?;

        let show_cmd = self.build_show_command();
        let show_output = crate::run::run_sync(&show_cmd)?;
        parse_commit_details(&show_output)
    }
}
