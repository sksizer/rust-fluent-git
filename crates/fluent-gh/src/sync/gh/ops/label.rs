use crate::error::LabelError;
use crate::ops::label::{parse_create_output, parse_delete_output, parse_edit_output, parse_list_output};
use crate::ops::{LabelCreateBuilder, LabelDeleteBuilder, LabelEditBuilder, LabelListBuilder};
use crate::types::LabelInfo;

#[cfg(not(feature = "tokio"))]
impl<'a> LabelCreateBuilder<'a> {
    pub fn run(self) -> Result<(), LabelError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &name, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> LabelListBuilder<'a> {
    pub fn run(self) -> Result<Vec<LabelInfo>, LabelError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> LabelEditBuilder<'a> {
    pub fn run(self) -> Result<(), LabelError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_edit_output(&output, &name, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> LabelDeleteBuilder<'a> {
    pub fn run(self) -> Result<(), LabelError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &name, &slug)
    }
}
