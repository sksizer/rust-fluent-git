use crate::error::DiskError;
use crate::ops::disk::{
    DiskCreateBuilder, DiskDeleteBuilder, DiskListBuilder, DiskResizeBuilder, parse_create_output, parse_delete_output,
    parse_list_output, parse_resize_output,
};
use crate::types::DiskInfo;

#[cfg(feature = "blocking")]
impl DiskCreateBuilder {
    pub fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl DiskListBuilder {
    pub fn run(self) -> Result<Vec<DiskInfo>, DiskError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl DiskDeleteBuilder {
    pub fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl DiskResizeBuilder {
    pub fn run(self) -> Result<(), DiskError> {
        let cmd = self.build_command();
        let name = self.disk_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_resize_output(&output, &name)
    }
}
