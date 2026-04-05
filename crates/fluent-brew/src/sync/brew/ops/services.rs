use crate::error::ServiceError;
use crate::ops::services::{
    ServicesInfoBuilder, ServicesKillBuilder, ServicesListBuilder, ServicesRestartBuilder, ServicesRunBuilder,
    ServicesStartBuilder, ServicesStopBuilder, parse_services_info_output, parse_services_kill_output,
    parse_services_list_output, parse_services_restart_output, parse_services_run_output, parse_services_start_output,
    parse_services_stop_output,
};
use crate::types::ServiceInfo;

#[cfg(not(feature = "tokio"))]
impl ServicesListBuilder {
    pub fn run(self) -> Result<Vec<ServiceInfo>, ServiceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_list_output(&output)
    }
}

#[cfg(not(feature = "tokio"))]
impl ServicesInfoBuilder {
    pub fn run(self) -> Result<Vec<ServiceInfo>, ServiceError> {
        let cmd = self.build_command();
        let name = self.service_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_info_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl ServicesStartBuilder {
    pub fn run(self) -> Result<(), ServiceError> {
        let cmd = self.build_command();
        let name = self.service_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_start_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl ServicesStopBuilder {
    pub fn run(self) -> Result<(), ServiceError> {
        let cmd = self.build_command();
        let name = self.service_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_stop_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl ServicesRestartBuilder {
    pub fn run(self) -> Result<(), ServiceError> {
        let cmd = self.build_command();
        let name = self.service_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_restart_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl ServicesRunBuilder {
    pub fn run(self) -> Result<(), ServiceError> {
        let cmd = self.build_command();
        let name = self.service_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_run_output(&output, &name)
    }
}

#[cfg(not(feature = "tokio"))]
impl ServicesKillBuilder {
    pub fn run(self) -> Result<(), ServiceError> {
        let cmd = self.build_command();
        let name = self.service_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_services_kill_output(&output, &name)
    }
}
