use crate::error::AddError;
use crate::ops::AddBuilder;
use crate::ops::add::{parse_add_output, parse_staged_files};
use crate::types::AddResult;

#[cfg(not(feature = "tokio"))]
impl<'a> AddBuilder<'a> {
    pub fn run(self) -> Result<AddResult, AddError> {
        let add_cmd = self.build_add_command();
        let output = fluent_core::run_sync(&add_cmd)?;
        parse_add_output(&output, self.paths_ref())?;

        let diff_cmd = self.build_diff_command();
        let diff_output = fluent_core::run_sync(&diff_cmd)?;
        Ok(parse_staged_files(&diff_output))
    }
}
