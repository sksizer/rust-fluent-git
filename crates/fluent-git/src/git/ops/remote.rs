use crate::error::RemoteError;
use crate::ops::remote::{parse_add_output, parse_list_output, parse_remove_output};
use crate::ops::{RemoteAddBuilder, RemoteListBuilder, RemoteRemoveBuilder};
use crate::types::RemoteInfo;

#[cfg(not(feature = "blocking"))]
impl<'a> RemoteAddBuilder<'a> {
    pub async fn run(self) -> Result<(), RemoteError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_add_output(&output, self.name())
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> RemoteRemoveBuilder<'a> {
    pub async fn run(self) -> Result<(), RemoteError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_remove_output(&output, self.name())
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> RemoteListBuilder<'a> {
    pub async fn run(self) -> Result<Vec<RemoteInfo>, RemoteError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}
