use crate::error::QueryError;
use crate::ops::query::{
    DepsBuilder, InfoBuilder, ListBuilder, OutdatedBuilder, SearchBuilder, parse_deps_output, parse_info_output,
    parse_list_output, parse_outdated_output, parse_search_output,
};
use crate::types::{InfoResponse, OutdatedResponse};

#[cfg(feature = "blocking")]
impl InfoBuilder {
    pub fn run(self) -> Result<InfoResponse, QueryError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_sync(&cmd)?;
        parse_info_output(&output, &name)
    }
}

#[cfg(feature = "blocking")]
impl SearchBuilder {
    pub fn run(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_search_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl ListBuilder {
    pub fn run(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl OutdatedBuilder {
    pub fn run(self) -> Result<OutdatedResponse, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_outdated_output(&output)
    }
}

#[cfg(feature = "blocking")]
impl DepsBuilder {
    pub fn run(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_sync(&cmd)?;
        parse_deps_output(&output)
    }
}
