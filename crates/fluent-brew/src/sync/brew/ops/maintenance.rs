use crate::error::MaintenanceError;
use crate::ops::maintenance::{
    AutoremoveBuilder, CleanupBuilder, DoctorBuilder, UpdateBuilder, parse_autoremove_output, parse_cleanup_output,
    parse_doctor_output, parse_update_output,
};

#[cfg(feature = "blocking")]
impl UpdateBuilder {
    pub fn run(self) -> Result<(), MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_update_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl CleanupBuilder {
    pub fn run(self) -> Result<(), MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_cleanup_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl AutoremoveBuilder {
    pub fn run(self) -> Result<(), MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_autoremove_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl DoctorBuilder {
    pub fn run(self) -> Result<String, MaintenanceError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_doctor_output(&output)
    }
}
