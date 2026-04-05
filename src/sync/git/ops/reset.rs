use crate::error::ResetError;
use crate::ops::ResetBuilder;
use crate::ops::reset::{parse_reset_output, parse_rev_parse_for_reset};
use crate::types::ResetResult;

#[cfg(not(feature = "tokio"))]
impl<'a> ResetBuilder<'a> {
    pub fn run(self) -> Result<ResetResult, ResetError> {
        let cmd = self.build_reset_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_reset_output(&output, &self.target_ref())?;

        let rev_cmd = self.build_rev_parse_command();
        let rev_output = crate::run::run_sync(&rev_cmd)?;
        parse_rev_parse_for_reset(&rev_output, self.mode())
    }
}
