use crate::error::ResetError;
use crate::ops::ResetBuilder;
use crate::ops::reset::{parse_reset_output, parse_rev_parse_for_reset};
use crate::types::ResetResult;

#[cfg(feature = "tokio")]
impl<'a> ResetBuilder<'a> {
    pub async fn run_async(self) -> Result<ResetResult, ResetError> {
        let cmd = self.build_reset_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_reset_output(&output, &self.target_ref())?;

        let rev_cmd = self.build_rev_parse_command();
        let rev_output = fluent_core::run_async(&rev_cmd).await?;
        parse_rev_parse_for_reset(&rev_output, self.mode())
    }
}
