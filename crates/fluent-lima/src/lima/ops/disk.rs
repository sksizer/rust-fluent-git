use crate::error::DiskError;
use crate::ops::disk::{
    DiskCreateBuilder, DiskDeleteBuilder, DiskListBuilder, DiskResizeBuilder, parse_create_output, parse_delete_output,
    parse_list_output, parse_resize_output,
};
use crate::types::DiskInfo;

#[cfg(not(feature = "blocking"))]
impl DiskCreateBuilder {
    pub async fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &name)
    }
}

#[cfg(not(feature = "blocking"))]
impl DiskListBuilder {
    pub async fn run(self) -> Result<Vec<DiskInfo>, DiskError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}

#[cfg(not(feature = "blocking"))]
impl DiskDeleteBuilder {
    pub async fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, &name)
    }
}

#[cfg(not(feature = "blocking"))]
impl DiskResizeBuilder {
    pub async fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_resize_output(&output, &name)
    }
}
