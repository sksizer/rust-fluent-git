use crate::error::AddError;
use crate::ops::AddBuilder;
use crate::ops::add::{parse_add_output, parse_staged_files};
use crate::types::AddResult;

#[cfg(not(feature = "blocking"))]
impl<'a> AddBuilder<'a> {
    pub async fn run(self) -> Result<AddResult, AddError> {
        let add_cmd = self.build_add_command();
        let output = crate::run::run_async(&add_cmd).await?;
        parse_add_output(&output, self.paths_ref())?;

        let diff_cmd = self.build_diff_command();
        let diff_output = crate::run::run_async(&diff_cmd).await?;
        Ok(parse_staged_files(&diff_output))
    }
}
