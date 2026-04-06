use crate::error::RebaseError;
use crate::ops::RebaseBuilder;
use crate::ops::rebase::parse_rebase_output;

#[cfg(feature = "tokio")]
impl<'a> RebaseBuilder<'a> {
    pub async fn run_async(self) -> Result<(), RebaseError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_rebase_output(&output, self.onto_ref())
    }
}
