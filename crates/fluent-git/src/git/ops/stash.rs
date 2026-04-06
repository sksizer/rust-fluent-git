use crate::error::StashError;
use crate::ops::stash::{parse_list_output, parse_pop_output, parse_push_output};
use crate::ops::{StashListBuilder, StashPopBuilder, StashPushBuilder};
use crate::types::StashEntry;

#[cfg(feature = "tokio")]
impl<'a> StashPushBuilder<'a> {
    pub async fn run_async(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_push_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl<'a> StashPopBuilder<'a> {
    pub async fn run_async(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_pop_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl<'a> StashListBuilder<'a> {
    pub async fn run_async(self) -> Result<Vec<StashEntry>, StashError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}
