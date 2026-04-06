use crate::error::AuthError;
use crate::ops::AuthStatusBuilder;
use crate::ops::auth::parse_status_output;
use crate::types::AuthStatus;

#[cfg(feature = "blocking")]
impl AuthStatusBuilder {
    pub fn run(self) -> Result<AuthStatus, AuthError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_status_output(&output)
    }
}
