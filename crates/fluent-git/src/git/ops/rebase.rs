use crate::error::RebaseError;
use crate::ops::RebaseBuilder;
use crate::ops::rebase::parse_rebase_output;

#[cfg(not(feature = "blocking"))]
impl<'a> RebaseBuilder<'a> {
    pub async fn run(self) -> Result<(), RebaseError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_rebase_output(&output, self.onto_ref())
    }
}
