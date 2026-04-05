//! Builders for `limactl` instance lifecycle operations.

use std::process::Output;

use cmd_spec::ShellCommand;

use crate::error::InstanceError;
use crate::types::InstanceInfo;
use fluent_core::{CommandError, stderr_string, stdout_string};

// ── Helper ───────────────────────────────────────────────────────────

/// Classify stderr into a typed error, falling back to a generic command error.
fn classify_stderr(stderr: &str, name: &str, args: &str, output: &Output) -> InstanceError {
    let lower = stderr.to_lowercase();

    if lower.contains("does not exist") || lower.contains("not found") {
        return InstanceError::NotFound { name: name.to_string() };
    }

    if lower.contains("already exists") {
        return InstanceError::AlreadyExists { name: name.to_string() };
    }

    if lower.contains("already running") {
        return InstanceError::AlreadyRunning { name: name.to_string() };
    }

    if lower.contains("already stopped") {
        return InstanceError::AlreadyStopped { name: name.to_string() };
    }

    if lower.contains("protected") {
        return InstanceError::Protected { name: name.to_string() };
    }

    InstanceError::Command(CommandError::Failed {
        args: args.to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr: stderr.to_string(),
    })
}

// ── Create ───────────────────────────────────────────────────────────

pub struct CreateBuilder {
    name: Option<String>,
    template: Option<String>,
    cpus: Option<u32>,
    memory_gib: Option<f32>,
    disk_gib: Option<f32>,
    arch: Option<String>,
    vm_type: Option<String>,
    mounts: Vec<(String, Option<String>, bool)>,
    mount_type: Option<String>,
    containerd: Option<String>,
    networks: Vec<String>,
    port_forwards: Vec<String>,
    plain: bool,
    rosetta: bool,
    set_expressions: Vec<String>,
}

impl CreateBuilder {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            template: None,
            cpus: None,
            memory_gib: None,
            disk_gib: None,
            arch: None,
            vm_type: None,
            mounts: Vec::new(),
            mount_type: None,
            containerd: None,
            networks: Vec::new(),
            port_forwards: Vec::new(),
            plain: false,
            rosetta: false,
            set_expressions: Vec::new(),
        }
    }

    /// Set the instance name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the template (e.g. "template:docker", a URL, or a file path).
    pub fn template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }

    /// Set the number of CPUs.
    pub fn cpus(mut self, cpus: u32) -> Self {
        self.cpus = Some(cpus);
        self
    }

    /// Set memory in GiB.
    pub fn memory_gib(mut self, gib: f32) -> Self {
        self.memory_gib = Some(gib);
        self
    }

    /// Set disk size in GiB.
    pub fn disk_gib(mut self, gib: f32) -> Self {
        self.disk_gib = Some(gib);
        self
    }

    /// Set the architecture (e.g. "aarch64", "x86_64").
    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = Some(arch.into());
        self
    }

    /// Set the VM type (e.g. "vz", "qemu").
    pub fn vm_type(mut self, vm_type: impl Into<String>) -> Self {
        self.vm_type = Some(vm_type.into());
        self
    }

    /// Add a mount (host_path, optional mount_point, writable).
    pub fn mount(mut self, host_path: impl Into<String>, mount_point: Option<String>, writable: bool) -> Self {
        self.mounts.push((host_path.into(), mount_point, writable));
        self
    }

    /// Set the mount type (e.g. "reverse-sshfs", "virtiofs").
    pub fn mount_type(mut self, mount_type: impl Into<String>) -> Self {
        self.mount_type = Some(mount_type.into());
        self
    }

    /// Set containerd mode (e.g. "system", "user", "none").
    pub fn containerd(mut self, mode: impl Into<String>) -> Self {
        self.containerd = Some(mode.into());
        self
    }

    /// Add a network (can be called multiple times).
    pub fn network(mut self, network: impl Into<String>) -> Self {
        self.networks.push(network.into());
        self
    }

    /// Add a port forward rule (can be called multiple times).
    pub fn port_forward(mut self, rule: impl Into<String>) -> Self {
        self.port_forwards.push(rule.into());
        self
    }

    /// Enable plain mode.
    pub fn plain(mut self) -> Self {
        self.plain = true;
        self
    }

    /// Enable Rosetta.
    pub fn rosetta(mut self) -> Self {
        self.rosetta = true;
        self
    }

    /// Add a `--set` yq override expression (can be called multiple times).
    pub fn set(mut self, expression: impl Into<String>) -> Self {
        self.set_expressions.push(expression.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("limactl").arg("create").arg("--tty=false");

        if let Some(ref name) = self.name {
            cmd = cmd.arg("--name").arg(name);
        }

        if let Some(ref cpus) = self.cpus {
            cmd = cmd.arg("--cpus").arg(cpus.to_string());
        }

        if let Some(ref memory) = self.memory_gib {
            cmd = cmd.arg("--memory").arg(format!("{memory}GiB"));
        }

        if let Some(ref disk) = self.disk_gib {
            cmd = cmd.arg("--disk").arg(format!("{disk}GiB"));
        }

        if let Some(ref arch) = self.arch {
            cmd = cmd.arg("--arch").arg(arch);
        }

        if let Some(ref vm_type) = self.vm_type {
            cmd = cmd.arg("--vm-type").arg(vm_type);
        }

        for (host_path, mount_point, writable) in &self.mounts {
            let mut mount_spec = host_path.clone();
            if let Some(mp) = mount_point {
                mount_spec.push(':');
                mount_spec.push_str(mp);
            }
            if *writable {
                mount_spec.push_str(":w");
            }
            cmd = cmd.arg("--mount").arg(mount_spec);
        }

        if let Some(ref mount_type) = self.mount_type {
            cmd = cmd.arg("--mount-type").arg(mount_type);
        }

        if let Some(ref containerd) = self.containerd {
            cmd = cmd.arg("--containerd").arg(containerd);
        }

        for network in &self.networks {
            cmd = cmd.arg("--network").arg(network);
        }

        for rule in &self.port_forwards {
            cmd = cmd.arg("--port-forward").arg(rule);
        }

        if self.plain {
            cmd = cmd.arg("--plain");
        }

        if self.rosetta {
            cmd = cmd.arg("--rosetta");
        }

        for expr in &self.set_expressions {
            cmd = cmd.arg("--set").arg(expr);
        }

        // Template is the last positional argument
        if let Some(ref template) = self.template {
            cmd = cmd.arg(template);
        }

        cmd
    }

    pub(crate) fn instance_name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}

pub(crate) fn parse_create_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl create {name}"), output))
}

// ── Start ────────────────────────────────────────────────────────────

pub struct StartBuilder {
    name: String,
}

impl StartBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("start").arg("--tty=false").arg(&self.name)
    }

    pub(crate) fn instance_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_start_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl start {name}"), output))
}

// ── Stop ─────────────────────────────────────────────────────────────

pub struct StopBuilder {
    name: String,
}

impl StopBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("stop").arg(&self.name)
    }

    pub(crate) fn instance_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_stop_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl stop {name}"), output))
}

// ── Restart ──────────────────────────────────────────────────────────

pub struct RestartBuilder {
    name: String,
}

impl RestartBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("restart").arg(&self.name)
    }

    pub(crate) fn instance_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_restart_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl restart {name}"), output))
}

// ── Delete ───────────────────────────────────────────────────────────

pub struct DeleteBuilder {
    name: String,
    force: bool,
}

impl DeleteBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), force: false }
    }

    /// Force deletion (skip confirmation, remove even if running).
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("limactl").arg("delete");

        if self.force {
            cmd = cmd.arg("--force");
        }

        cmd = cmd.arg(&self.name);
        cmd
    }

    pub(crate) fn instance_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_delete_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl delete {name}"), output))
}

// ── List ─────────────────────────────────────────────────────────────

pub struct ListBuilder {
    filter: Vec<String>,
}

impl ListBuilder {
    pub(crate) fn new() -> Self {
        Self { filter: Vec::new() }
    }

    /// Add a filter expression (can be called multiple times).
    pub fn filter(mut self, expr: impl Into<String>) -> Self {
        self.filter.push(expr.into());
        self
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        let mut cmd = ShellCommand::new("limactl").arg("list").arg("--json").arg("--tty=false");

        for f in &self.filter {
            cmd = cmd.arg("--filter").arg(f);
        }

        cmd
    }
}

pub(crate) fn parse_list_output(output: &Output) -> Result<Vec<InstanceInfo>, InstanceError> {
    if output.status.success() {
        let stdout = stdout_string(output);
        let mut instances = Vec::new();
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let info: InstanceInfo = serde_json::from_str(trimmed)?;
            instances.push(info);
        }
        return Ok(instances);
    }

    let stderr = stderr_string(output);
    Err(InstanceError::Command(CommandError::Failed {
        args: "limactl list --json".to_string(),
        code: output.status.code().unwrap_or(-1),
        stdout: stdout_string(output),
        stderr,
    }))
}

// ── Clone ────────────────────────────────────────────────────────────

pub struct CloneBuilder {
    source: String,
    destination: String,
}

impl CloneBuilder {
    pub(crate) fn new(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self { source: source.into(), destination: destination.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("clone").arg(&self.source).arg(&self.destination)
    }

    pub(crate) fn source_name(&self) -> &str {
        &self.source
    }
}

pub(crate) fn parse_clone_output(output: &Output, source: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, source, &format!("limactl clone {source}"), output))
}

// ── Rename ───────────────────────────────────────────────────────────

pub struct RenameBuilder {
    old_name: String,
    new_name: String,
}

impl RenameBuilder {
    pub(crate) fn new(old_name: impl Into<String>, new_name: impl Into<String>) -> Self {
        Self { old_name: old_name.into(), new_name: new_name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("rename").arg(&self.old_name).arg(&self.new_name)
    }

    pub(crate) fn old_name(&self) -> &str {
        &self.old_name
    }
}

pub(crate) fn parse_rename_output(output: &Output, old_name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, old_name, &format!("limactl rename {old_name}"), output))
}

// ── Protect ──────────────────────────────────────────────────────────

pub struct ProtectBuilder {
    name: String,
}

impl ProtectBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("protect").arg(&self.name)
    }

    pub(crate) fn instance_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_protect_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl protect {name}"), output))
}

// ── Unprotect ────────────────────────────────────────────────────────

pub struct UnprotectBuilder {
    name: String,
}

impl UnprotectBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn build_command(&self) -> ShellCommand {
        ShellCommand::new("limactl").arg("unprotect").arg(&self.name)
    }

    pub(crate) fn instance_name(&self) -> &str {
        &self.name
    }
}

pub(crate) fn parse_unprotect_output(output: &Output, name: &str) -> Result<(), InstanceError> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = stderr_string(output);
    Err(classify_stderr(&stderr, name, &format!("limactl unprotect {name}"), output))
}
