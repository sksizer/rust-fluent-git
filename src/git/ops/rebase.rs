use crate::error::RebaseError;
use crate::ops::rebase::parse_rebase_output;
use crate::ops::RebaseBuilder;

#[cfg(not(feature = "blocking"))]
impl<'a> RebaseBuilder<'a> {
    pub async fn run(self) -> Result<(), RebaseError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_rebase_output(&output, self.onto_ref())
    }
}
