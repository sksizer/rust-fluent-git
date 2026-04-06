use crate::error::PromptError;
use crate::ops::prompt::{PromptBuilder, parse_prompt_output};
use crate::types::PromptResult;

#[cfg(feature = "tokio")]
impl PromptBuilder {
    pub async fn run_async(self) -> Result<PromptResult, PromptError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_prompt_output(&output)
    }
}
