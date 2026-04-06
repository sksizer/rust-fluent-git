use crate::error::QueryError;
use crate::ops::query::{
    DepsBuilder, InfoBuilder, ListBuilder, OutdatedBuilder, SearchBuilder, parse_deps_output, parse_info_output,
    parse_list_output, parse_outdated_output, parse_search_output,
};
use crate::types::{InfoResponse, OutdatedResponse};

#[cfg(feature = "tokio")]
impl InfoBuilder {
    pub async fn run_async(self) -> Result<InfoResponse, QueryError> {
        let cmd = self.build_command();
        let name = self.formula_name().to_string();
        let output = fluent_core::run_async(&cmd).await?;
        parse_info_output(&output, &name)
    }
}

#[cfg(feature = "tokio")]
impl SearchBuilder {
    pub async fn run_async(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_search_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl ListBuilder {
    pub async fn run_async(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_list_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl OutdatedBuilder {
    pub async fn run_async(self) -> Result<OutdatedResponse, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_outdated_output(&output)
    }
}

#[cfg(feature = "tokio")]
impl DepsBuilder {
    pub async fn run_async(self) -> Result<Vec<String>, QueryError> {
        let cmd = self.build_command();
        let output = fluent_core::run_async(&cmd).await?;
        parse_deps_output(&output)
    }
}
