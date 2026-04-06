use crate::error::LabelError;
use crate::ops::label::{parse_create_output, parse_delete_output, parse_edit_output, parse_list_output};
use crate::ops::{LabelCreateBuilder, LabelDeleteBuilder, LabelEditBuilder, LabelListBuilder};
use crate::types::LabelInfo;

#[cfg(feature = "tokio")]
impl<'a> LabelCreateBuilder<'a> {
    pub async fn run_async(self) -> Result<(), LabelError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &name, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> LabelListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<LabelInfo>, LabelError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> LabelEditBuilder<'a> {
    pub async fn run_async(self) -> Result<(), LabelError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_edit_output(&output, &name, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> LabelDeleteBuilder<'a> {
    pub async fn run_async(self) -> Result<(), LabelError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, &name, &slug)
    }
}
