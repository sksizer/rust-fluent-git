use crate::error::QueryError;
use crate::ops::query::{
    DepsBuilder, InfoBuilder, ListBuilder, OutdatedBuilder, SearchBuilder, parse_deps_output, parse_info_output,
    parse_list_output, parse_outdated_output, parse_search_output,
};
use crate::types::{InfoResponse, OutdatedResponse};

#[cfg(not(feature = "blocking"))]
impl InfoBuilder {
    pub async fn run(self) -> Result<InfoResponse, QueryError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_info_output(&output, &name)
    }
}

#[cfg(not(feature = "blocking"))]
impl SearchBuilder {
    pub async fn run(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_search_output(&output)
    }
}

#[cfg(not(feature = "blocking"))]
impl ListBuilder {
    pub async fn run(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}

#[cfg(not(feature = "blocking"))]
impl OutdatedBuilder {
    pub async fn run(self) -> Result<OutdatedResponse, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_outdated_output(&output)
    }
}

#[cfg(not(feature = "blocking"))]
impl DepsBuilder {
    pub async fn run(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_deps_output(&output)
    }
}
