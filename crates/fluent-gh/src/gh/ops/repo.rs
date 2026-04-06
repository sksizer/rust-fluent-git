use crate::error::RepoError;
use crate::ops::repo::{parse_clone_output, parse_create_output, parse_fork_output, parse_view_output};
use crate::ops::{RepoCloneBuilder, RepoCreateBuilder, RepoForkBuilder, RepoViewBuilder};
use crate::types::{RepoCreateResult, RepoInfo};

#[cfg(feature = "tokio")]
impl<'a> RepoViewBuilder<'a> {
    pub async fn run_async(self) -> Result<RepoInfo, RepoError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_view_output(&output, "")
    }
}

#[cfg(feature = "tokio")]
impl<'a> RepoCloneBuilder<'a> {
    pub async fn run_async(self) -> Result<(), RepoError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_clone_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> RepoCreateBuilder<'a> {
    pub async fn run_async(self) -> Result<RepoCreateResult, RepoError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> RepoForkBuilder<'a> {
    pub async fn run_async(self) -> Result<(), RepoError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_fork_output(&output, &slug)
    }
}
