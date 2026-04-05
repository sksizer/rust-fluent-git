use crate::error::CleanError;
use crate::ops::CleanBuilder;
use crate::ops::clean::parse_clean_output;
use crate::types::CleanResult;

#[cfg(not(feature = "tokio"))]
impl<'a> CleanBuilder<'a> {
    pub fn run(self) -> Result<CleanResult, CleanError> {
        if !self.is_force() {
            return Err(CleanError::ForceRequired);
        }

        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_clean_output(&output)
    }
}
