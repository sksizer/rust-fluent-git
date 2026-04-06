use crate::error::LogError;
use crate::ops::LogBuilder;
use crate::ops::log::parse_log_output;
use crate::types::LogEntry;

#[cfg(feature = "tokio")]
impl<'a> LogBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<LogEntry>, LogError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_log_output(&output)
    }
}
