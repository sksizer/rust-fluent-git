use crate::error::SecretError;
use crate::ops::secret::{parse_delete_output, parse_list_output, parse_set_output};
use crate::ops::{SecretDeleteBuilder, SecretListBuilder, SecretSetBuilder};
use crate::types::SecretInfo;

#[cfg(feature = "tokio")]
impl<'a> SecretSetBuilder<'a> {
    pub async fn run_async(self) -> Result<(), SecretError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_set_output(&output, &name, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> SecretListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<SecretInfo>, SecretError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> SecretDeleteBuilder<'a> {
    pub async fn run_async(self) -> Result<(), SecretError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, &name, &slug)
    }
}
