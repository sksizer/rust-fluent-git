use crate::error::StatusError;
use crate::ops::StatusBuilder;
use crate::ops::status::parse_status_output;
use crate::types::StatusResult;

#[cfg(feature = "blocking")]
impl<'a> StatusBuilder<'a> {
    pub fn run(self) -> Result<StatusResult, StatusError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_status_output(&output)
    }
}
