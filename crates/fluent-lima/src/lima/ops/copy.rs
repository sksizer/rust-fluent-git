use crate::error::CopyError;
use crate::ops::copy::{CopyBuilder, parse_copy_output};

#[cfg(feature = "tokio")]
impl CopyBuilder {
    pub async fn run_async(self) -> Result<(), CopyError> {
        let cmd = self.build_command();
        let name = self.instance_name_hint();
        let output = fluent_core::run_async(&cmd).await?;
        parse_copy_output(&output, &name)
    }
}
