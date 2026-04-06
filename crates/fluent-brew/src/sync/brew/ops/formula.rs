use crate::error::FormulaError;
use crate::ops::formula::{
    InstallBuilder, LinkBuilder, PinBuilder, ReinstallBuilder, UninstallBuilder, UnlinkBuilder, UnpinBuilder,
    UpgradeBuilder, parse_install_output, parse_link_output, parse_pin_output, parse_reinstall_output,
    parse_uninstall_output, parse_unlink_output, parse_unpin_output, parse_upgrade_output,
};

#[cfg(feature = "blocking")]
impl InstallBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_install_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl UninstallBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_uninstall_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl ReinstallBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_reinstall_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl UpgradeBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().map(|s| s.to_string());
        let output = fluent_core::run_sync(&cmd)?;
        parse_upgrade_output(&output, name.as_deref())
    }
}

#[cfg(feature = "blocking")]
impl PinBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_pin_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl UnpinBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_unpin_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl LinkBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_link_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl UnlinkBuilder {
    pub fn run(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_unlink_output(&output, &name)
    }
}
