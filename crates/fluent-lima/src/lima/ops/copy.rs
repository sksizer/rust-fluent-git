use crate::error::CopyError;
use crate::ops::copy::{CopyBuilder, parse_copy_output};

#[cfg(not(feature = "blocking"))]
impl CopyBuilder {
    pub async fn run(self) -> Result<(), CopyError> {
        let cmd = self.build_command();
        let name = self.instance_name_hint();
        let output = fluent_core::run_async(&cmd).await?;
        parse_copy_output(&output, &name)
    }
}
