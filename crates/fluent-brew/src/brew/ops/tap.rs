use crate::error::TapError;
use crate::ops::tap::{TapBuilder, UntapBuilder, parse_tap_output, parse_untap_output};

#[cfg(feature = "tokio")]
impl TapBuilder {
    pub async fn run_async(self) -> Result<(), TapError> {
        let cmd = self.build_command();
        let name = self.tap_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_tap_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl UntapBuilder {
    pub async fn run_async(self) -> Result<(), TapError> {
        let cmd = self.build_command();
        let name = self.tap_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_untap_output(&output, &name)
    }
}
