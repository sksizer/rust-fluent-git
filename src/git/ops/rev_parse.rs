use crate::error::RevParseError;
use crate::ops::rev_parse::parse_rev_parse_output;
use crate::ops::RevParseBuilder;

#[cfg(not(feature = "blocking"))]
impl<'a> RevParseBuilder<'a> {
    pub async fn run(self) -> Result<String, RevParseError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_rev_parse_output(&output, self.reference())
    }
}
