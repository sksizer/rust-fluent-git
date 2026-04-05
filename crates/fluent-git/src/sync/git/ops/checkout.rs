use crate::error::CheckoutError;
use crate::ops::CheckoutBranchBuilder;
use crate::ops::checkout::parse_checkout_output;

#[cfg(not(feature = "tokio"))]
impl<'a> CheckoutBranchBuilder<'a> {
    pub fn run(self) -> Result<(), CheckoutError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_checkout_output(&output, self.name(), self.is_create())
    }
}
