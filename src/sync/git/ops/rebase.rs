use crate::error::RebaseError;
use crate::ops::rebase::parse_rebase_output;
use crate::ops::RebaseBuilder;

#[cfg(not(feature = "tokio"))]
impl<'a> RebaseBuilder<'a> {
    pub fn run(self) -> Result<(), RebaseError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_rebase_output(&output, self.onto_ref())
    }
}
