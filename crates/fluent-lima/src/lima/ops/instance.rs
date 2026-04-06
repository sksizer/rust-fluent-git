use crate::error::InstanceError;
use crate::ops::instance::{
    CloneBuilder, CreateBuilder, DeleteBuilder, ListBuilder, ProtectBuilder, RenameBuilder, RestartBuilder,
    StartBuilder, StopBuilder, UnprotectBuilder, parse_clone_output, parse_create_output, parse_delete_output,
    parse_list_output, parse_protect_output, parse_rename_output, parse_restart_output, parse_start_output,
    parse_stop_output, parse_unprotect_output,
};
use crate::types::InstanceInfo;

#[cfg(feature = "tokio")]
impl CreateBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name();
        let output = fluent_core::run_async(&cmd).await?;
        parse_create_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl StartBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_start_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl StopBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_stop_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl RestartBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_restart_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl DeleteBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_delete_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl ListBuilder {
    pub async fn run_async(self) -> Result<Vec<InstanceInfo>, InstanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl CloneBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let source = self.source_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_clone_output(&output, &source)
    }
}

#[cfg(feature = "tokio")]
impl RenameBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let old = self.old_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_rename_output(&output, &old)
    }
}

#[cfg(feature = "tokio")]
impl ProtectBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_protect_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl UnprotectBuilder {
    pub async fn run_async(self) -> Result<(), InstanceError> {
        let cmd = self.build_command();
        let name = self.instance_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_unprotect_output(&output, &name)
    }
}
