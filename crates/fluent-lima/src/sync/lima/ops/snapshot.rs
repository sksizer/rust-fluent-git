use crate::error::SnapshotError;
use crate::ops::snapshot::{
    SnapshotApplyBuilder, SnapshotCreateBuilder, SnapshotDeleteBuilder, SnapshotListBuilder,
    parse_snapshot_list_output, parse_snapshot_output,
};
use crate::types::SnapshotInfo;

#[cfg(feature = "blocking")]
impl SnapshotCreateBuilder {
    pub fn run(self) -> Result<(), SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_snapshot_output(&output, &self.instance, Some(&self.tag))
    }
}

#[cfg(feature = "blocking")]
impl SnapshotApplyBuilder {
    pub fn run(self) -> Result<(), SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_snapshot_output(&output, &self.instance, Some(&self.tag))
    }
}

#[cfg(feature = "blocking")]
impl SnapshotDeleteBuilder {
    pub fn run(self) -> Result<(), SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_snapshot_output(&output, &self.instance, Some(&self.tag))
    }
}

#[cfg(feature = "blocking")]
impl SnapshotListBuilder {
    pub fn run(self) -> Result<Vec<SnapshotInfo>, SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_snapshot_list_output(&output, &self.instance)
    }
}
