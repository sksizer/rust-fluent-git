use crate::error::MaintenanceError;
use crate::ops::maintenance::{
    AutoremoveBuilder, CleanupBuilder, DoctorBuilder, UpdateBuilder, parse_autoremove_output, parse_cleanup_output,
    parse_doctor_output, parse_update_output,
};

#[cfg(feature = "tokio")]
impl UpdateBuilder {
    pub async fn run_async(self) -> Result<(), MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_update_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl CleanupBuilder {
    pub async fn run_async(self) -> Result<(), MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_cleanup_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl AutoremoveBuilder {
    pub async fn run_async(self) -> Result<(), MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_autoremove_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl DoctorBuilder {
    pub async fn run_async(self) -> Result<String, MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_doctor_output(&output)
    }
}
