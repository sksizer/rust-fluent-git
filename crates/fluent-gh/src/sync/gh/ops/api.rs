use crate::error::ApiError;
use crate::ops::ApiBuilder;
use crate::ops::api::parse_api_output;

#[cfg(feature = "blocking")]
impl<'a> ApiBuilder<'a> {
    pub fn run(self) -> Result<String, ApiError> {
        let endpoint = self.endpoint().to_string();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_api_output(&output, &endpoint)
    }
}
