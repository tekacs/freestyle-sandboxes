//! Fluent builder for VM creation requests.
//!
//! Mirrors the JS SDK's `VmSpec` builder, providing a chainable API for
//! constructing `CreateVmRequest` payloads with systemd service generation,
//! Dockerfile base images, git repo mounting, and more.
//!
//! # Examples
//!
//! ```no_run
//! use freestyle_sandboxes::VmSpecBuilder;
//!
//! let request = VmSpecBuilder::new()
//!     .rootfs_size_gb(4)
//!     .mem_size_gb(2)
//!     .vcpu_count(2)
//!     .apt_deps(["git", "curl", "nodejs"])
//!     .workdir("/app")
//!     .run_commands(["npm install", "npm run build"])
//!     .build();
//! ```

use crate::openapi::types::*;
use std::collections::HashMap;

const RUN_COMMANDS_PREFIX: &str = "freestyle-run-command";
const WAIT_FOR_PREFIX: &str = "freestyle-wait-for";

/// Fluent builder for `CreateVmRequest`.
#[derive(Default)]
pub struct VmSpecBuilder {
    req: CreateVmRequest,
}

impl VmSpecBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the final `CreateVmRequest`.
    pub fn build(self) -> CreateVmRequest {
        self.req
    }

    // --- Resource configuration ---

    pub fn rootfs_size_gb(mut self, gb: i64) -> Self {
        self.template_mut().rootfs_size_gb = Some(gb);
        self
    }

    pub fn mem_size_gb(mut self, gb: i64) -> Self {
        self.template_mut().mem_size_gb = Some(gb);
        self
    }

    pub fn vcpu_count(mut self, count: i32) -> Self {
        self.template_mut().vcpu_count = Some(count);
        self
    }

    pub fn idle_timeout_seconds(mut self, seconds: i64) -> Self {
        self.req.idle_timeout_seconds = Some(seconds);
        self
    }

    pub fn ready_signal_timeout_seconds(mut self, seconds: i64) -> Self {
        self.req.ready_signal_timeout_seconds = Some(seconds);
        self
    }

    pub fn wait_for_ready_signal(mut self, wait: bool) -> Self {
        self.req.wait_for_ready_signal = Some(wait);
        self
    }

    pub fn workdir(mut self, path: impl Into<String>) -> Self {
        self.req.workdir = Some(path.into());
        self
    }

    pub fn persistence(mut self, persistence: VmPersistence) -> Self {
        self.req.persistence = Some(persistence);
        self
    }

    pub fn recreate(mut self, recreate: bool) -> Self {
        self.req.recreate = Some(recreate);
        self
    }

    pub fn discriminator(mut self, name: impl Into<String>) -> Self {
        self.template_mut().discriminator = Some(name.into());
        self
    }

    pub fn snapshot_id(mut self, id: impl Into<String>) -> Self {
        self.req.snapshot_id = Some(SnapshotId(id.into()));
        self
    }

    // --- Base image ---

    /// Set the base image via Dockerfile content.
    pub fn base_image(mut self, dockerfile: impl Into<String>) -> Self {
        self.template_mut().base_image = Some(BaseImageSpec {
            dockerfile_content: dockerfile.into(),
        });
        self
    }

    /// Set the base image from a `VmBaseImageBuilder`.
    pub fn base_image_spec(mut self, spec: BaseImageSpec) -> Self {
        self.template_mut().base_image = Some(spec);
        self
    }

    // --- Packages ---

    /// Add apt packages to install.
    pub fn apt_deps(mut self, deps: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let existing = self.template_mut().apt_deps.get_or_insert_with(Vec::new);
        existing.extend(deps.into_iter().map(Into::into));
        self
    }

    // --- Users and groups ---

    pub fn users(mut self, users: impl IntoIterator<Item = LinuxUserSpec>) -> Self {
        let existing = self.req.users.get_or_insert_with(Vec::new);
        existing.extend(users);
        self
    }

    pub fn groups(mut self, groups: impl IntoIterator<Item = LinuxGroupSpec>) -> Self {
        let existing = self.req.groups.get_or_insert_with(Vec::new);
        existing.extend(groups);
        self
    }

    // --- Files ---

    /// Add files to be written into the VM.
    pub fn additional_files(mut self, files: HashMap<String, FreestyleFile>) -> Self {
        let existing = self.req.additional_files.get_or_insert_with(HashMap::new);
        existing.extend(files);
        self
    }

    /// Add a single file.
    pub fn additional_file(mut self, path: impl Into<String>, content: impl Into<String>) -> Self {
        let existing = self.req.additional_files.get_or_insert_with(HashMap::new);
        existing.insert(
            path.into(),
            FreestyleFile {
                content: content.into(),
                encoding: None,
                executable: None,
            },
        );
        self
    }

    // --- Git ---

    /// Mount a Freestyle git repo into the VM.
    pub fn repo(mut self, repo_id: impl Into<String>, path: impl Into<String>) -> Self {
        let git = self.req.git.get_or_insert(GitOptions {
            config: GitConfig { user: None },
            repos: None,
        });
        let repos = git.repos.get_or_insert_with(Vec::new);
        repos.push(GitRepositorySpec {
            repo: repo_id.into(),
            path: path.into(),
            rev: None,
        });
        self
    }

    // --- Ports ---

    pub fn ports(mut self, ports: impl IntoIterator<Item = PortMapping>) -> Self {
        let existing = self.req.ports.get_or_insert_with(Vec::new);
        existing.extend(ports);
        self
    }

    /// Expose a single port mapping (external_port → target_port inside VM).
    pub fn port(mut self, external: i32, target: i32) -> Self {
        let existing = self.req.ports.get_or_insert_with(Vec::new);
        existing.push(PortMapping {
            port: external,
            target_port: target,
        });
        self
    }

    // --- Domains ---

    pub fn domains(mut self, domains: impl IntoIterator<Item = VmDomainConfig>) -> Self {
        let existing = self.req.domains.get_or_insert_with(Vec::new);
        existing.extend(domains);
        self
    }

    // --- Systemd services ---

    /// Add a custom systemd service spec.
    pub fn systemd_service(mut self, spec: SystemdUnitSpec) -> Self {
        let systemd = self.req.systemd.get_or_insert_with(SystemdConfig::default);
        let services = systemd.services.get_or_insert_with(Vec::new);
        services.push(spec);
        self
    }

    /// Add shell commands to run as chained oneshot systemd services.
    ///
    /// Each command gets its own service that depends on the previous one,
    /// ensuring sequential execution.
    pub fn run_commands(mut self, commands: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let systemd = self.req.systemd.get_or_insert_with(SystemdConfig::default);
        let services = systemd.services.get_or_insert_with(Vec::new);

        let (mut max_idx, mut last_name) = generated_service_state(services, RUN_COMMANDS_PREFIX);

        for cmd in commands {
            let cmd = cmd.into();
            let trimmed = cmd.trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            max_idx += 1;
            let name = format!("{RUN_COMMANDS_PREFIX}-{max_idx}");
            let script_path = format!("/opt/freestyle/scripts/{name}.sh");
            let script = if trimmed.starts_with("#!") {
                trimmed
            } else {
                format!("#!/bin/bash\n{trimmed}")
            };

            // Add script as additional file
            let files = self.req.additional_files.get_or_insert_with(HashMap::new);
            files.insert(
                script_path.clone(),
                FreestyleFile {
                    content: script,
                    encoding: Some("utf-8".to_string()),
                    executable: None,
                },
            );

            let mut after = None;
            let mut requires = None;
            if let Some(prev) = &last_name {
                after = Some(vec![prev.clone()]);
                requires = Some(vec![prev.clone()]);
            }

            let spec = SystemdUnitSpec {
                name: name.clone(),
                mode: SystemdUnitMode::Oneshot,
                delete_after_success: Some(true),
                exec: vec![format!("/bin/bash {script_path}")],
                after,
                requires,
                enable: None,
                env: None,
                group: None,
                on_failure: None,
                ready_signal: None,
                remain_after_exit: None,
                restart_policy: None,
                timeout_sec: None,
                user: None,
                wanted_by: None,
                watchdog_sec: None,
                workdir: None,
            };

            let systemd = self.req.systemd.get_or_insert_with(SystemdConfig::default);
            let services = systemd.services.get_or_insert_with(Vec::new);
            services.push(spec);
            last_name = Some(name);
        }
        self
    }

    /// Add a polling wait-for command that blocks until the command succeeds.
    pub fn wait_for(mut self, command: impl Into<String>, interval_seconds: u32) -> Self {
        let command = command.into();
        let trimmed = command.trim().to_string();
        assert!(!trimmed.is_empty(), "wait_for requires a non-empty command");
        assert!(interval_seconds > 0, "interval_seconds must be > 0");

        let systemd = self.req.systemd.get_or_insert_with(SystemdConfig::default);
        let services = systemd.services.get_or_insert_with(Vec::new);

        let (max_idx, last_name) = generated_service_state(services, WAIT_FOR_PREFIX);
        let name = format!("{WAIT_FOR_PREFIX}-{}", max_idx + 1);

        let body = trimmed
            .lines()
            .map(|l| format!("    {l}"))
            .collect::<Vec<_>>()
            .join("\n");
        let script = format!(
            "#!/bin/bash\nwhile true; do\n  if (\n{body}\n  ); then\n    exit 0\n  fi\n  sleep {interval_seconds}\ndone"
        );
        let script_path = format!("/opt/freestyle/scripts/{name}.sh");

        let files = self.req.additional_files.get_or_insert_with(HashMap::new);
        files.insert(
            script_path.clone(),
            FreestyleFile {
                content: script,
                encoding: Some("utf-8".to_string()),
                executable: None,
            },
        );

        let mut after = None;
        let mut requires = None;
        if let Some(prev) = &last_name {
            after = Some(vec![prev.clone()]);
            requires = Some(vec![prev.clone()]);
        }

        let spec = SystemdUnitSpec {
            name: name.clone(),
            mode: SystemdUnitMode::Oneshot,
            timeout_sec: Some(0),
            exec: vec![format!("/bin/bash {script_path}")],
            after,
            requires,
            delete_after_success: None,
            enable: None,
            env: None,
            group: None,
            on_failure: None,
            ready_signal: None,
            remain_after_exit: None,
            restart_policy: None,
            user: None,
            wanted_by: None,
            watchdog_sec: None,
            workdir: None,
        };

        let systemd = self.req.systemd.get_or_insert_with(SystemdConfig::default);
        let services = systemd.services.get_or_insert_with(Vec::new);
        services.push(spec);
        self
    }

    // --- Template helper ---

    fn template_mut(&mut self) -> &mut VmTemplate {
        self.req.template.get_or_insert_with(VmTemplate::default)
    }
}

/// Scan existing services for generated service names and return
/// (max_index, last_generated_name).
fn generated_service_state(services: &[SystemdUnitSpec], prefix: &str) -> (usize, Option<String>) {
    let all_prefixes = [RUN_COMMANDS_PREFIX, WAIT_FOR_PREFIX];
    let mut max_idx = 0usize;
    let mut last_generated: Option<String> = None;

    for svc in services {
        for gen_prefix in &all_prefixes {
            if let Some(rest) = svc.name.strip_prefix(gen_prefix)
                && let Some(rest) = rest.strip_prefix('-')
                && let Ok(idx) = rest.parse::<usize>()
            {
                if *gen_prefix == prefix && idx > max_idx {
                    max_idx = idx;
                }
                last_generated = Some(svc.name.clone());
            }
        }
    }
    (max_idx, last_generated)
}

/// Construct a `VmBaseImage` (Dockerfile content) from layers.
///
/// # Examples
///
/// ```
/// let dockerfile = freestyle_sandboxes::vm_base_image("debian:trixie-slim")
///     .run("apt-get update && apt-get install -y nodejs")
///     .run("npm install -g pnpm")
///     .build();
/// ```
pub struct VmBaseImageBuilder {
    layers: Vec<String>,
}

pub fn vm_base_image(from: impl Into<String>) -> VmBaseImageBuilder {
    let from = from.into();
    let line = if from.trim().to_lowercase().starts_with("from ") {
        from.trim().to_string()
    } else {
        format!("FROM {}", from.trim())
    };
    VmBaseImageBuilder { layers: vec![line] }
}

impl VmBaseImageBuilder {
    /// Add a RUN instruction.
    pub fn run(mut self, command: impl Into<String>) -> Self {
        let cmd = command.into();
        let trimmed = cmd.trim();
        if !trimmed.is_empty() {
            self.layers
                .push(format!("RUN <<'FREESTYLE_EOF'\n{trimmed}\nFREESTYLE_EOF"));
        }
        self
    }

    /// Add raw Dockerfile content.
    pub fn append(mut self, dockerfile: impl Into<String>) -> Self {
        let content = dockerfile.into();
        let trimmed = content.trim();
        if !trimmed.is_empty() {
            self.layers.push(trimmed.to_string());
        }
        self
    }

    /// Build the Dockerfile content string.
    pub fn build(self) -> String {
        self.layers
            .into_iter()
            .filter(|l| !l.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Build into a `BaseImageSpec`.
    pub fn into_spec(self) -> BaseImageSpec {
        BaseImageSpec {
            dockerfile_content: self.build(),
        }
    }
}
