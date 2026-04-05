//! Builder for `limactl info`.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::InfoError;
use crate::types::SystemInfo;
use fluent_core::{CommandError, stderr_string, stdout_string};

// ── InfoBuilder ─────────────────────────────────────────────────────

pub struct InfoBuilder;

impl InfoBuilder {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("info").arg("--tty=false")
    }
}

pub(crate) fn parse_info_output(output: &Output) -> Result<SystemInfo, InfoError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let info: SystemInfo = serde_json::from_str(&stdout)?;
        return Ok(info);
    }

    let stderr = stderr_string(output);
    Err(InfoError::Command(CommandError::Failed {
        args: "limactl info".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
