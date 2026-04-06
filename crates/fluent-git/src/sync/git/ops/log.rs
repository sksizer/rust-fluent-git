use crate::error::LogError;
use crate::ops::LogBuilder;
use crate::ops::log::parse_log_output;
use crate::types::LogEntry;

#[cfg(feature = "blocking")]
impl<'a> LogBuilder<'a> {
    pub fn run(self) -> Result<Vec<LogEntry>, LogError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_log_output(&output)
    }
}
