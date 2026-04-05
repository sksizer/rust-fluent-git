use crate::error::TapError;
use crate::ops::tap::{TapBuilder, UntapBuilder, parse_tap_output, parse_untap_output};

#[cfg(not(feature = "blocking"))]
impl TapBuilder {
    pub async fn run(self) -> Result<(), TapError> {
        let cmd = self.build_command();
        let name = self.tap_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_tap_output(&output, &name)
    }
}

#[cfg(not(feature = "blocking"))]
impl UntapBuilder {
    pub async fn run(self) -> Result<(), TapError> {
        let cmd = self.build_command();
        let name = self.tap_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_untap_output(&output, &name)
    }
}
