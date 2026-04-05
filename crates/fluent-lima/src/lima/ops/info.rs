use crate::error::InfoError;
use crate::ops::info::{InfoBuilder, parse_info_output};
use crate::types::SystemInfo;

#[cfg(not(feature = "blocking"))]
impl InfoBuilder {
    pub async fn run(self) -> Result<SystemInfo, InfoError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_info_output(&output)
    }
}
