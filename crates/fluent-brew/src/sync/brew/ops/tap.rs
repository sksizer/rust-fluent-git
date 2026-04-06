use crate::error::TapError;
use crate::ops::tap::{TapBuilder, UntapBuilder, parse_tap_output, parse_untap_output};

#[cfg(feature = "blocking")]
impl TapBuilder {
    pub fn run(self) -> Result<(), TapError> {
        let cmd = self.build_command();
        let name = self.tap_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_tap_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl UntapBuilder {
    pub fn run(self) -> Result<(), TapError> {
        let cmd = self.build_command();
        let name = self.tap_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_untap_output(&output, &name)
    }
}
