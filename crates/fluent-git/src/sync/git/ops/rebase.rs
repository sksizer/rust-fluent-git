use crate::error::RebaseError;
use crate::ops::RebaseBuilder;
use crate::ops::rebase::parse_rebase_output;

#[cfg(not(feature = "tokio"))]
impl<'a> RebaseBuilder<'a> {
    pub fn run(self) -> Result<(), RebaseError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_rebase_output(&output, self.onto_ref())
    }
}
