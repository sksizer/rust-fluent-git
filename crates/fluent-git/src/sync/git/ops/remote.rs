use crate::error::RemoteError;
use crate::ops::remote::{parse_add_output, parse_list_output, parse_remove_output};
use crate::ops::{RemoteAddBuilder, RemoteListBuilder, RemoteRemoveBuilder};
use crate::types::RemoteInfo;

#[cfg(feature = "blocking")]
impl<'a> RemoteAddBuilder<'a> {
    pub fn run(self) -> Result<(), RemoteError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_add_output(&output, self.name())
    }
}

#[cfg(feature = "blocking")]
impl<'a> RemoteRemoveBuilder<'a> {
    pub fn run(self) -> Result<(), RemoteError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_remove_output(&output, self.name())
    }
}

#[cfg(feature = "blocking")]
impl<'a> RemoteListBuilder<'a> {
    pub fn run(self) -> Result<Vec<RemoteInfo>, RemoteError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}
