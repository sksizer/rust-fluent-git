use crate::error::StatusError;
use crate::ops::StatusBuilder;
use crate::ops::status::parse_status_output;
use crate::types::StatusResult;

#[cfg(feature = "tokio")]
impl<'a> StatusBuilder<'a> {
    pub async fn run_async(self) -> Result<StatusResult, StatusError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_status_output(&output)
    }
}
