use crate::error::InstanceError;
use crate::ops::instance::{
    CloneBuilder, CreateBuilder, DeleteBuilder, ListBuilder, ProtectBuilder, RenameBuilder, RestartBuilder,
    StartBuilder, StopBuilder, UnprotectBuilder, parse_clone_output, parse_create_output, parse_delete_output,
    parse_list_output, parse_protect_output, parse_rename_output, parse_restart_output, parse_start_output,
    parse_stop_output, parse_unprotect_output,
};
use crate::types::InstanceInfo;

#[cfg(feature = "blocking")]
impl CreateBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name();
        let output = fluent_core::run_sync(&cmd)?;
        parse_create_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl StartBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_start_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl StopBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_stop_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl RestartBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_restart_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl DeleteBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_delete_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl ListBuilder {
    pub fn run(self) -> Result<Vec<InstanceInfo>, InstanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl CloneBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let source = self.source_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_clone_output(&output, &source)
    }
}

#[cfg(feature = "blocking")]
impl RenameBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let old = self.old_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_rename_output(&output, &old)
    }
}

#[cfg(feature = "blocking")]
impl ProtectBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_protect_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl UnprotectBuilder {
    pub fn run(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_unprotect_output(&output, &name)
    }
}
