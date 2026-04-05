use crate::error::TagError;
use crate::ops::tag::{parse_create_output, parse_delete_output, parse_list_output};
use crate::ops::{TagCreateBuilder, TagDeleteBuilder, TagListBuilder};
use crate::types::TagInfo;

#[cfg(not(feature = "tokio"))]
impl<'a> TagCreateBuilder<'a> {
    pub fn run(self) -> Result<(), TagError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, self.name())
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> TagDeleteBuilder<'a> {
    pub fn run(self) -> Result<(), TagError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, self.name())
    }
}

#[cfg(not(feature = "tokio"))]
impl<'a> TagListBuilder<'a> {
    pub fn run(self) -> Result<Vec<TagInfo>, TagError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}
