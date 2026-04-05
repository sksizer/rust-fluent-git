use crate::error::PromptError;
use crate::ops::prompt::{PromptBuilder, parse_prompt_output};
use crate::types::PromptResult;

#[cfg(not(feature = "tokio"))]
impl PromptBuilder {
    pub fn run(self) -> Result<PromptResult, PromptError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_prompt_output(&output)
    }
}
