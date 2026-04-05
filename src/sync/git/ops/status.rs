use crate::error::StatusError;
use crate::ops::status::parse_status_output;
use crate::ops::StatusBuilder;
use crate::types::StatusResult;

#[cfg(not(feature = "tokio"))]
impl<'a> StatusBuilder<'a> {
    pub fn run(self) -> Result<StatusResult, StatusError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_status_output(&output)
    }
}
