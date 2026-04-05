use crate::error::StashError;
use crate::ops::stash::{parse_push_output, parse_pop_output, parse_list_output};
use crate::ops::{StashPushBuilder, StashPopBuilder, StashListBuilder};
use crate::types::StashEntry;

#[cfg(not(feature = "blocking"))]
impl<'a> StashPushBuilder<'a> {
    pub async fn run(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_push_output(&output)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> StashPopBuilder<'a> {
    pub async fn run(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_pop_output(&output)
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> StashListBuilder<'a> {
    pub async fn run(self) -> Result<Vec<StashEntry>, StashError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}
