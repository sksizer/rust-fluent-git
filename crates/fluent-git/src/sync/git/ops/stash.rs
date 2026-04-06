use crate::error::StashError;
use crate::ops::stash::{parse_list_output, parse_pop_output, parse_push_output};
use crate::ops::{StashListBuilder, StashPopBuilder, StashPushBuilder};
use crate::types::StashEntry;

#[cfg(feature = "blocking")]
impl<'a> StashPushBuilder<'a> {
    pub fn run(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_push_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl<'a> StashPopBuilder<'a> {
    pub fn run(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_pop_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl<'a> StashListBuilder<'a> {
    pub fn run(self) -> Result<Vec<StashEntry>, StashError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}
