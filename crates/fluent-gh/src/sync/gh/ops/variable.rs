use crate::error::VariableError;
use crate::ops::variable::{parse_delete_output, parse_get_output, parse_list_output, parse_set_output};
use crate::ops::{VariableDeleteBuilder, VariableGetBuilder, VariableListBuilder, VariableSetBuilder};
use crate::types::VariableInfo;

#[cfg(feature = "blocking")]
impl<'a> VariableSetBuilder<'a> {
    pub fn run(self) -> Result<(), VariableError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_set_output(&output, &name, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> VariableListBuilder<'a> {
    pub fn run(self) -> Result<Vec<VariableInfo>, VariableError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> VariableDeleteBuilder<'a> {
    pub fn run(self) -> Result<(), VariableError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &name, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> VariableGetBuilder<'a> {
    pub fn run(self) -> Result<VariableInfo, VariableError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_get_output(&output, &name, &slug)
    }
}
