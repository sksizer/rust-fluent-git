use crate::error::PrError;
use crate::ops::pr::{
    parse_checkout_output, parse_close_output, parse_create_output, parse_list_output, parse_merge_output,
    parse_view_output,
};
use crate::ops::{PrCheckoutBuilder, PrCloseBuilder, PrCreateBuilder, PrListBuilder, PrMergeBuilder, PrViewBuilder};
use crate::types::{PrCreateResult, PrInfo};

#[cfg(not(feature = "tokio"))]
impl<'a> PrCreateBuilder<'a> {
    pub fn run(self) -> Result<PrCreateResult, PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> PrListBuilder<'a> {
    pub fn run(self) -> Result<Vec<PrInfo>, PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> PrViewBuilder<'a> {
    pub fn run(self) -> Result<PrInfo, PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_view_output(&output, number, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> PrMergeBuilder<'a> {
    pub fn run(self) -> Result<(), PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_merge_output(&output, number, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> PrCloseBuilder<'a> {
    pub fn run(self) -> Result<(), PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_close_output(&output, number, &slug)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> PrCheckoutBuilder<'a> {
    pub fn run(self) -> Result<(), PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_checkout_output(&output, &slug)
    }
}
