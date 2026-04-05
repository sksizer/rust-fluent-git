use crate::error::ConfigError;
use crate::ops::config::{parse_config_get_output, parse_config_set_output, parse_config_unset_output};
use crate::ops::{ConfigGetBuilder, ConfigSetBuilder, ConfigUnsetBuilder};

#[cfg(not(feature = "blocking"))]
impl<'a> ConfigSetBuilder<'a> {
    pub async fn run(self) -> Result<(), ConfigError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_config_set_output(&output, self.key())
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> ConfigGetBuilder<'a> {
    pub async fn run(self) -> Result<String, ConfigError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_config_get_output(&output, self.key())
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> ConfigUnsetBuilder<'a> {
    pub async fn run(self) -> Result<(), ConfigError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_config_unset_output(&output, self.key())
    }
}
