use crate::error::CleanError;
use crate::ops::clean::parse_clean_output;
use crate::ops::CleanBuilder;
use crate::types::CleanResult;

#[cfg(not(feature = "blocking"))]
impl<'a> CleanBuilder<'a> {
    pub async fn run(self) -> Result<CleanResult, CleanError> {
        if !self.is_force() {
            return Err(CleanError::ForceRequired);
        }

        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_clean_output(&output)
    }
}
