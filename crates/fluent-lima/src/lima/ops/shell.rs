use crate::error::ShellError;
use crate::ops::shell::{ShellBuilder, parse_shell_output};
use crate::types::ShellResult;

#[cfg(feature = "tokio")]
impl ShellBuilder {
    pub async fn run_async(self) -> Result<ShellResult, ShellError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_shell_output(&output, &name)
    }
}
