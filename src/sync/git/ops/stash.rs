use crate::error::StashError;
use crate::ops::stash::{parse_push_output, parse_pop_output, parse_list_output};
use crate::ops::{StashPushBuilder, StashPopBuilder, StashListBuilder};
use crate::types::StashEntry;

#[cfg(not(feature = "tokio"))]
impl<'a> StashPushBuilder<'a> {
    pub fn run(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_push_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> StashPopBuilder<'a> {
    pub fn run(self) -> Result<(), StashError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_pop_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> StashListBuilder<'a> {
    pub fn run(self) -> Result<Vec<StashEntry>, StashError> {
        let cmd = self.build_command();
        let output = crate::run::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}
