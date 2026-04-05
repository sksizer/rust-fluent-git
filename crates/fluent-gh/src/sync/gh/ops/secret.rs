use crate::error::SecretError;
use crate::ops::secret::{parse_delete_output, parse_list_output, parse_set_output};
use crate::ops::{SecretDeleteBuilder, SecretListBuilder, SecretSetBuilder};
use crate::types::SecretInfo;

#[cfg(not(feature = "tokio"))]
impl<'a> SecretSetBuilder<'a> {
    pub fn run(self) -> Result<(), SecretError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_set_output(&output, &name, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> SecretListBuilder<'a> {
    pub fn run(self) -> Result<Vec<SecretInfo>, SecretError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> SecretDeleteBuilder<'a> {
    pub fn run(self) -> Result<(), SecretError> {
        let name = self.name().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &name, &slug)
    }
}
