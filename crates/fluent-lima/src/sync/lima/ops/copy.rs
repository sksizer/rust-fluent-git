use crate::error::CopyError;
use crate::ops::copy::{CopyBuilder, parse_copy_output};

#[cfg(feature = "blocking")]
impl CopyBuilder {
    pub fn run(self) -> Result<(), CopyError> {
        let cmd = self.build_command();
        let name = self.instance_name_hint();
        let output = fluent_core::run_sync(&cmd)?;
        parse_copy_output(&output, &name)
    }
}
