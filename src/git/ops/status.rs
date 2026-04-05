use crate::error::StatusError;
use crate::ops::StatusBuilder;
use crate::ops::status::parse_status_output;
use crate::types::StatusResult;

#[cfg(not(feature = "blocking"))]
impl<'a> StatusBuilder<'a> {
    pub async fn run(self) -> Result<StatusResult, StatusError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_status_output(&output)
    }
}
