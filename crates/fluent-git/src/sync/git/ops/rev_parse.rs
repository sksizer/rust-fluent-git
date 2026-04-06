use crate::error::RevParseError;
use crate::ops::RevParseBuilder;
use crate::ops::rev_parse::parse_rev_parse_output;

#[cfg(feature = "blocking")]
impl<'a> RevParseBuilder<'a> {
    pub fn run(self) -> Result<String, RevParseError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_rev_parse_output(&output, self.reference())
    }
}
