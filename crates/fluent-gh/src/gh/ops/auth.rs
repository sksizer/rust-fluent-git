use crate::error::AuthError;
use crate::ops::AuthStatusBuilder;
use crate::ops::auth::parse_status_output;
use crate::types::AuthStatus;

#[cfg(not(feature = "blocking"))]
impl<'a> AuthStatusBuilder<'a> {
    pub async fn run(self) -> Result<AuthStatus, AuthError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_status_output(&output)
    }
}
