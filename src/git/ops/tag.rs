use crate::error::TagError;
use crate::ops::tag::{parse_create_output, parse_delete_output, parse_list_output};
use crate::ops::{TagCreateBuilder, TagDeleteBuilder, TagListBuilder};
use crate::types::TagInfo;

#[cfg(not(feature = "blocking"))]
impl<'a> TagCreateBuilder<'a> {
    pub async fn run(self) -> Result<(), TagError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_create_output(&output, self.name())
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> TagDeleteBuilder<'a> {
    pub async fn run(self) -> Result<(), TagError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_delete_output(&output, self.name())
    }
}

#[cfg(not(feature = "blocking"))]
impl<'a> TagListBuilder<'a> {
    pub async fn run(self) -> Result<Vec<TagInfo>, TagError> {
        let cmd = self.build_command();
        let output = crate::run::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}
