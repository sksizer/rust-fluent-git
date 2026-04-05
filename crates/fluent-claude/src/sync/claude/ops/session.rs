use crate::error::SessionError;
use crate::ops::SessionResumeBuilder;
use crate::ops::session::parse_resume_output;
use crate::types::PromptResult;

#[cfg(not(feature = "tokio"))]
impl SessionResumeBuilder {
    pub fn run(self) -> Result<PromptResult, SessionError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_resume_output(&output)
    }
}
