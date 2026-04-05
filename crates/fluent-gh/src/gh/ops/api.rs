use crate::error::ApiError;
use crate::ops::ApiBuilder;
use crate::ops::api::parse_api_output;

#[cfg(not(feature = "blocking"))]
impl<'a> ApiBuilder<'a> {
    pub async fn run(self) -> Result<String, ApiError> {
        let endpoint = self.endpoint().to_string();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_api_output(&output, &endpoint)
    }
}
