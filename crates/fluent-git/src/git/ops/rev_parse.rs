use crate::error::RevParseError;
use crate::ops::RevParseBuilder;
use crate::ops::rev_parse::parse_rev_parse_output;

#[cfg(feature = "tokio")]
impl<'a> RevParseBuilder<'a> {
    pub async fn run_async(self) -> Result<String, RevParseError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_rev_parse_output(&output, self.reference())
    }
}
