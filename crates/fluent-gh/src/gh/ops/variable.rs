use crate::error::VariableError;
use crate::ops::variable::{parse_delete_output, parse_get_output, parse_list_output, parse_set_output};
use crate::ops::{VariableDeleteBuilder, VariableGetBuilder, VariableListBuilder, VariableSetBuilder};
use crate::types::VariableInfo;

#[cfg(feature = "tokio")]
impl<'a> VariableSetBuilder<'a> {
    pub async fn run_async(self) -> Result<(), VariableError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_set_output(&output, &name, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> VariableListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<VariableInfo>, VariableError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> VariableDeleteBuilder<'a> {
    pub async fn run_async(self) -> Result<(), VariableError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, &name, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> VariableGetBuilder<'a> {
    pub async fn run_async(self) -> Result<VariableInfo, VariableError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_get_output(&output, &name, &slug)
    }
}
