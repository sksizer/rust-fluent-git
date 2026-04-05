use crate::error::PrError;
use crate::ops::pr::{
    parse_checkout_output, parse_close_output, parse_create_output, parse_list_output, parse_merge_output,
    parse_view_output,
};
use crate::ops::{PrCheckoutBuilder, PrCloseBuilder, PrCreateBuilder, PrListBuilder, PrMergeBuilder, PrViewBuilder};
use crate::types::{PrCreateResult, PrInfo};

#[cfg(not(feature = "blocking"))]
impl<'a> PrCreateBuilder<'a> {
    pub async fn run(self) -> Result<PrCreateResult, PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> PrListBuilder<'a> {
    pub async fn run(self) -> Result<Vec<PrInfo>, PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> PrViewBuilder<'a> {
    pub async fn run(self) -> Result<PrInfo, PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_view_output(&output, number, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> PrMergeBuilder<'a> {
    pub async fn run(self) -> Result<(), PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_merge_output(&output, number, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> PrCloseBuilder<'a> {
    pub async fn run(self) -> Result<(), PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_close_output(&output, number, &slug)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> PrCheckoutBuilder<'a> {
    pub async fn run(self) -> Result<(), PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_checkout_output(&output, &slug)
    }
}
