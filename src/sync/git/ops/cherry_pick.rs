use crate::error::CherryPickError;
use crate::ops::cherry_pick::parse_cherry_pick_output;
use crate::ops::CherryPickBuilder;

#[cfg(not(feature = "tokio"))]
impl<'a> CherryPickBuilder<'a> {
    pub fn run(self) -> Result<(), CherryPickError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_cherry_pick_output(&output, self.sha_ref())
    }
}
