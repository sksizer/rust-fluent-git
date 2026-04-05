use crate::error::CheckoutError;
use crate::ops::checkout::parse_checkout_output;
use crate::ops::CheckoutBranchBuilder;

#[cfg(not(feature = "tokio"))]
impl<'a> CheckoutBranchBuilder<'a> {
    pub fn run(self) -> Result<(), CheckoutError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_checkout_output(&output, self.name(), self.is_create())
    }
}
