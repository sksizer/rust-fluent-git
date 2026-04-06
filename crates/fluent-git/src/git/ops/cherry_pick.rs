use crate::error::CherryPickError;
use crate::ops::CherryPickBuilder;
use crate::ops::cherry_pick::parse_cherry_pick_output;

#[cfg(feature = "tokio")]
impl<'a> CherryPickBuilder<'a> {
    pub async fn run_async(self) -> Result<(), CherryPickError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_cherry_pick_output(&output, self.sha_ref())
    }
}
