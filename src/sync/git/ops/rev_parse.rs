use crate::error::RevParseError;
use crate::ops::rev_parse::parse_rev_parse_output;
use crate::ops::RevParseBuilder;

#[cfg(not(feature = "tokio"))]
impl<'a> RevParseBuilder<'a> {
    pub fn run(self) -> Result<String, RevParseError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_rev_parse_output(&output, self.reference())
    }
}
