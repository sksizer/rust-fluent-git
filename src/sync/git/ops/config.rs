use crate::error::ConfigError;
use crate::ops::config::{parse_config_get_output, parse_config_set_output, parse_config_unset_output};
use crate::ops::{ConfigGetBuilder, ConfigSetBuilder, ConfigUnsetBuilder};

#[cfg(not(feature = "tokio"))]
impl<'a> ConfigSetBuilder<'a> {
    pub fn run(self) -> Result<(), ConfigError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_config_set_output(&output, self.key())
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> ConfigGetBuilder<'a> {
    pub fn run(self) -> Result<String, ConfigError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_config_get_output(&output, self.key())
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> ConfigUnsetBuilder<'a> {
    pub fn run(self) -> Result<(), ConfigError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_config_unset_output(&output, self.key())
    }
}
