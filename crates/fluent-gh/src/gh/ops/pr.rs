use crate::error::PrError;
use crate::ops::pr::{
    parse_checkout_output, parse_close_output, parse_create_output, parse_list_output, parse_merge_output,
    parse_view_output,
};
use crate::ops::{PrCheckoutBuilder, PrCloseBuilder, PrCreateBuilder, PrListBuilder, PrMergeBuilder, PrViewBuilder};
use crate::types::{PrCreateResult, PrInfo};

#[cfg(feature = "tokio")]
impl<'a> PrCreateBuilder<'a> {
    pub async fn run_async(self) -> Result<PrCreateResult, PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> PrListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<PrInfo>, PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> PrViewBuilder<'a> {
    pub async fn run_async(self) -> Result<PrInfo, PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_view_output(&output, number, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> PrMergeBuilder<'a> {
    pub async fn run_async(self) -> Result<(), PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_merge_output(&output, number, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> PrCloseBuilder<'a> {
    pub async fn run_async(self) -> Result<(), PrError> {
        let number = self.number();
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_close_output(&output, number, &slug)
    }
}

#[cfg(feature = "tokio")]
impl<'a> PrCheckoutBuilder<'a> {
    pub async fn run_async(self) -> Result<(), PrError> {
        let slug = self.repo_slug();
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_checkout_output(&output, &slug)
    }
}
