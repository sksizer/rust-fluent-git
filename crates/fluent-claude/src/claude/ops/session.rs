use crate::error::SessionError;
use crate::ops::SessionResumeBuilder;
use crate::ops::session::parse_resume_output;
use crate::types::PromptResult;

#[cfg(not(feature = "blocking"))]
impl SessionResumeBuilder {
    pub async fn run(self) -> Result<PromptResult, SessionError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_resume_output(&output)
    }
}
