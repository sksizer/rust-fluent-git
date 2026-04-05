use crate::error::DiffError;
use crate::ops::DiffBuilder;
use crate::ops::diff::{build_diff_result, check_diff_errors, parse_numstat_output};
use crate::types::DiffResult;

#[cfg(not(feature = "tokio"))]
impl<'a> DiffBuilder<'a> {
    pub fn run(self) -> Result<DiffResult, DiffError> {
        let numstat_cmd = self.build_numstat_command();
        let numstat_output = crate::run::run_sync(&numstat_cmd)?;
        let ref_range = self.ref_range();
        check_diff_errors(&numstat_output, ref_range.as_deref())?;
        let files = parse_numstat_output(&numstat_output)?;

        let raw_cmd = self.build_raw_command();
        let raw_output = crate::run::run_sync(&raw_cmd)?;

        Ok(build_diff_result(files, &raw_output))
    }
}
