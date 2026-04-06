use crate::error::SnapshotError;
use crate::ops::snapshot::{
    SnapshotApplyBuilder, SnapshotCreateBuilder, SnapshotDeleteBuilder, SnapshotListBuilder,
    parse_snapshot_list_output, parse_snapshot_output,
};
use crate::types::SnapshotInfo;

#[cfg(feature = "tokio")]
impl SnapshotCreateBuilder {
    pub async fn run_async(self) -> Result<(), SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_snapshot_output(&output, &self.instance, Some(&self.tag))
    }
}

#[cfg(feature = "tokio")]
impl SnapshotApplyBuilder {
    pub async fn run_async(self) -> Result<(), SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_snapshot_output(&output, &self.instance, Some(&self.tag))
    }
}

#[cfg(feature = "tokio")]
impl SnapshotDeleteBuilder {
    pub async fn run_async(self) -> Result<(), SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_snapshot_output(&output, &self.instance, Some(&self.tag))
    }
}

#[cfg(feature = "tokio")]
impl SnapshotListBuilder {
    pub async fn run_async(self) -> Result<Vec<SnapshotInfo>, SnapshotError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_snapshot_list_output(&output, &self.instance)
    }
}
