use fluent_core::CommandError;

#[derive(Debug, thiserror::Error)]
pub enum MaintenanceError {
    #[error(transparent)]
    Command(CommandError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<CommandError> for MaintenanceError {
    fn from(e: CommandError) -> Self {
        MaintenanceError::Command(e)
    }
}
