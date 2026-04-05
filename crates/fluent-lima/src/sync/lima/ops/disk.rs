use crate::error::DiskError;
use crate::ops::disk::{
    DiskCreateBuilder, DiskDeleteBuilder, DiskListBuilder, DiskResizeBuilder, parse_create_output, parse_delete_output,
    parse_list_output, parse_resize_output,
};
use crate::types::DiskInfo;

#[cfg(not(feature = "tokio"))]
impl DiskCreateBuilder {
    pub fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl DiskListBuilder {
    pub fn run(self) -> Result<Vec<DiskInfo>, DiskError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl DiskDeleteBuilder {
    pub fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl DiskResizeBuilder {
    pub fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_resize_output(&output, &name)
    }
}
