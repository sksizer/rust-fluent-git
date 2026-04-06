use crate::error::ReleaseError;
use crate::ops::release::{parse_create_output, parse_delete_output, parse_list_output, parse_view_output};
use crate::ops::{ReleaseCreateBuilder, ReleaseDeleteBuilder, ReleaseListBuilder, ReleaseViewBuilder};
use crate::types::{ReleaseCreateResult, ReleaseInfo};

#[cfg(feature = "tokio")]
impl<'a> ReleaseCreateBuilder<'a> {
    pub async fn run_async(self) -> Result<ReleaseCreateResult, ReleaseError> {
        let tag = self.tag().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &tag, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> ReleaseListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<ReleaseInfo>, ReleaseError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> ReleaseViewBuilder<'a> {
    pub async fn run_async(self) -> Result<ReleaseInfo, ReleaseError> {
        let tag = self.tag().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_view_output(&output, &tag, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> ReleaseDeleteBuilder<'a> {
    pub async fn run_async(self) -> Result<(), ReleaseError> {
        let tag = self.tag().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, &tag, &slug)
    }
}
