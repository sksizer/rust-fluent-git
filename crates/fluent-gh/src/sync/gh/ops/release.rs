use crate::error::ReleaseError;
use crate::ops::release::{parse_create_output, parse_delete_output, parse_list_output, parse_view_output};
use crate::ops::{ReleaseCreateBuilder, ReleaseDeleteBuilder, ReleaseListBuilder, ReleaseViewBuilder};
use crate::types::{ReleaseCreateResult, ReleaseInfo};

#[cfg(feature = "blocking")]
impl<'a> ReleaseCreateBuilder<'a> {
    pub fn run(self) -> Result<ReleaseCreateResult, ReleaseError> {
        let tag = self.tag().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &tag, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> ReleaseListBuilder<'a> {
    pub fn run(self) -> Result<Vec<ReleaseInfo>, ReleaseError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> ReleaseViewBuilder<'a> {
    pub fn run(self) -> Result<ReleaseInfo, ReleaseError> {
        let tag = self.tag().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_view_output(&output, &tag, &slug)
    }
}

#[cfg(feature = "blocking")]
impl<'a> ReleaseDeleteBuilder<'a> {
    pub fn run(self) -> Result<(), ReleaseError> {
        let tag = self.tag().to_string();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &tag, &slug)
    }
}
