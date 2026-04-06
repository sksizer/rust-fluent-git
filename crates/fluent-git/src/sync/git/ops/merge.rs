use crate::error::MergeError;
use crate::ops::MergeBuilder;
use crate::ops::merge::{parse_merge_details, parse_merge_output};
use crate::types::MergeResult;
use fluent_core::stdout_string;

#[cfg(feature = "blocking")]
impl<'a> MergeBuilder<'a> {
    pub fn run(self) -> Result<MergeResult, MergeError> {
        let cmd = self.build_merge_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_merge_output(&output, self.branch_ref())?;

        let merge_stdout = stdout_string(&output);

        let log_cmd = self.build_log_command();
        let log_output = fluent_core::run_sync(&log_cmd)?;
        parse_merge_details(&log_output, &merge_stdout)
    }
}
