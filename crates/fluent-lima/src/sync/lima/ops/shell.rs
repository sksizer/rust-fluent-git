use crate::error::ShellError;
use crate::ops::shell::{ShellBuilder, parse_shell_output};
use crate::types::ShellResult;

#[cfg(not(feature = "tokio"))]
impl ShellBuilder {
    pub fn run(self) -> Result<ShellResult, ShellError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_shell_output(&output, &name)
    }
}
