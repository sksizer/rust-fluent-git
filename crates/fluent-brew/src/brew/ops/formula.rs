use crate::error::FormulaError;
use crate::ops::formula::{
    InstallBuilder, LinkBuilder, PinBuilder, ReinstallBuilder, UninstallBuilder, UnlinkBuilder, UnpinBuilder,
    UpgradeBuilder, parse_install_output, parse_link_output, parse_pin_output, parse_reinstall_output,
    parse_uninstall_output, parse_unlink_output, parse_unpin_output, parse_upgrade_output,
};

#[cfg(feature = "tokio")]
impl InstallBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_install_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl UninstallBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_uninstall_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl ReinstallBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_reinstall_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl UpgradeBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().map(|s| s.to_string());
        let output = fluent_core::run_async(&cmd).await?;
        parse_upgrade_output(&output, name.as_deref())
    }
}

#[cfg(feature = "tokio")]
impl PinBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_pin_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl UnpinBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_unpin_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl LinkBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_link_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl UnlinkBuilder {
    pub async fn run_async(self) -> Result<(), FormulaError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_unlink_output(&output, &name)
    }
}
