use crate::error::RepoError;
use crate::ops::repo::{parse_clone_output, parse_create_output, parse_fork_output, parse_view_output};
use crate::ops::{RepoCloneBuilder, RepoCreateBuilder, RepoForkBuilder, RepoViewBuilder};
use crate::types::{RepoCreateResult, RepoInfo};

#[cfg(feature = "blocking")]
impl<'a> RepoViewBuilder<'a> {
    pub fn run(self) -> Result<RepoInfo, RepoError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_view_output(&output, "")
    }
}

#[cfg(feature = "blocking")]
impl<'a> RepoCloneBuilder<'a> {
    pub fn run(self) -> Result<(), RepoError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_clone_output(&output, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> RepoCreateBuilder<'a> {
    pub fn run(self) -> Result<RepoCreateResult, RepoError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> RepoForkBuilder<'a> {
    pub fn run(self) -> Result<(), RepoError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_fork_output(&output, &slug)
    }
}
