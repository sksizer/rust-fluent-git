use crate::error::SessionError;
use crate::ops::SessionResumeBuilder;
use crate::ops::session::parse_resume_output;
use crate::types::PromptResult;

#[cfg(feature = "tokio")]
impl SessionResumeBuilder {
    pub async fn run_async(self) -> Result<PromptResult, SessionError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_resume_output(&output)
    }
}
