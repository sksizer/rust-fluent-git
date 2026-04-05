use crate::error::CherryPickError;
use crate::ops::cherry_pick::parse_cherry_pick_output;
use crate::ops::CherryPickBuilder;

#[cfg(not(feature = "blocking"))]
impl<'a> CherryPickBuilder<'a> {
    pub async fn run(self) -> Result<(), CherryPickError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_cherry_pick_output(&output, self.sha_ref())
    }
}
