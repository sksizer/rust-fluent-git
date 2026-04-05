//! Builders for `gh api` operations.

use std::fmt;
use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::ApiError;
use crate::types::GitHub;
use fluent_core::CommandError;
use fluent_core::{stderr_string, stdout_string};

// ── HttpMethod enum ──────────────────────────────────────────────────

/// HTTP method for API requests.
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

// ── Entry Point ──────────────────────────────────────────────────────

/// Builder for `gh api` commands.
pub struct ApiBuilder<'a> {
    #[allow(dead_code)]
    gh: &'a GitHub,
    endpoint: String,
    method: Option<HttpMethod>,
    fields: Vec<(String, String)>,
    raw_fields: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    jq: Option<String>,
    paginate: bool,
}

impl<'a> ApiBuilder<'a> {
    pub(crate) fn new(gh: &'a GitHub, endpoint: impl Into<String>) -> Self {
        Self {
            gh,
            endpoint: endpoint.into(),
            method: None,
            fields: Vec::new(),
            raw_fields: Vec::new(),
            headers: Vec::new(),
            jq: None,
            paginate: false,
        }
    }

    /// Set the HTTP method (default is GET).
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Add a `-f key=value` field (can call multiple times).
    pub fn field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((key.into(), value.into()));
        self
    }

    /// Add a `--raw-field key=value` field (can call multiple times).
    pub fn raw_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.raw_fields.push((key.into(), value.into()));
        self
    }

    /// Add a `-H key:value` header (can call multiple times).
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Add a `--jq` expression.
    pub fn jq(mut self, expr: impl Into<String>) -> Self {
        self.jq = Some(expr.into());
        self
    }

    /// Enable `--paginate` flag.
    pub fn paginate(mut self) -> Self {
        self.paginate = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("gh").arg("api").arg(&self.endpoint);

        if let Some(method) = self.method {
            cmd = cmd.arg("--method").arg(method.to_string());
        }

        for (key, value) in &self.fields {
            cmd = cmd.arg("-f").arg(format!("{key}={value}"));
        }

        for (key, value) in &self.raw_fields {
            cmd = cmd.arg("--raw-field").arg(format!("{key}={value}"));
        }

        for (key, value) in &self.headers {
            cmd = cmd.arg("-H").arg(format!("{key}:{value}"));
        }

        if let Some(ref jq) = self.jq {
            cmd = cmd.arg("--jq").arg(jq);
        }

        if self.paginate {
            cmd = cmd.arg("--paginate");
        }

        cmd
    }

    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

pub(crate) fn parse_api_output(output: &Output, endpoint: &str) -> Result<String, ApiError> {
    if output.status.success() {
        return Ok(stdout_string(output));
    }

    let stderr = stderr_string(output);
    let lower = stderr.to_lowercase();

    if lower.contains("auth") || lower.contains("login") {
        return Err(ApiError::NotAuthenticated);
    }

    Err(ApiError::Command(CommandError::Failed {
        args: format!("api {endpoint}"),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}
