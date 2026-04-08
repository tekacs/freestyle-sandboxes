use clap::{Parser, Subcommand};
use freestyle_sandboxes::types::*;
use freestyle_sandboxes::{Freestyle, VmSpecBuilder};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "freestyle", about = "Freestyle Sandboxes CLI")]
struct Cli {
    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check authentication
    Whoami,
    /// VM management
    Vm {
        #[command(subcommand)]
        action: VmAction,
    },
    /// Serverless deployments
    Deploy {
        #[command(subcommand)]
        action: DeployAction,
    },
    /// Execute a one-off script
    Run {
        /// Inline code to execute
        #[arg(short, long, group = "source")]
        code: Option<String>,
        /// File containing code to execute
        #[arg(short, long, group = "source")]
        file: Option<String>,
        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,
    },
    /// Git repository management
    Git {
        #[command(subcommand)]
        action: GitAction,
    },
    /// Domain management
    Domain {
        #[command(subcommand)]
        action: DomainAction,
    },
    /// Cron schedule management
    Cron {
        #[command(subcommand)]
        action: CronAction,
    },
}

// --- VM ---

#[derive(Subcommand)]
enum VmAction {
    /// Create a new VM
    Create {
        /// VM name/discriminator
        #[arg(short, long)]
        name: Option<String>,
        /// Memory in GB
        #[arg(long, default_value = "1")]
        mem: i64,
        /// vCPU count
        #[arg(long, default_value = "1")]
        vcpus: i32,
        /// Rootfs size in GB
        #[arg(long, default_value = "2")]
        disk: i64,
        /// Snapshot ID to create from
        #[arg(short, long)]
        snapshot: Option<String>,
        /// Custom domain
        #[arg(short, long)]
        domain: Option<String>,
        /// Port to expose (TARGET or TARGET:EXTERNAL)
        #[arg(short, long, default_value = "3000")]
        port: String,
        /// APT packages to install
        #[arg(long)]
        apt: Vec<String>,
        /// Execute a command after creation
        #[arg(short = 'x', long)]
        exec: Option<String>,
        /// SSH into VM after creation
        #[arg(long)]
        ssh: bool,
        /// Delete VM after exec completes or when SSH session ends
        #[arg(long)]
        delete: bool,
    },
    /// List all VMs
    List,
    /// Get VM info
    Get {
        /// VM ID
        id: String,
    },
    /// Execute a command on a VM
    Exec {
        /// VM ID
        id: String,
        /// Command to execute
        command: String,
    },
    /// Start a VM
    Start {
        /// VM ID
        id: String,
    },
    /// Stop a VM
    Stop {
        /// VM ID
        id: String,
    },
    /// Delete one or more VMs
    Delete {
        /// VM IDs
        ids: Vec<String>,
    },
    /// SSH into a VM
    Ssh {
        /// VM ID
        id: String,
        /// Delete VM when SSH session ends
        #[arg(long)]
        delete: bool,
    },
    /// Create a snapshot of a VM
    Snapshot {
        /// VM ID
        id: String,
    },
}

// --- Deploy ---

#[derive(Subcommand)]
enum DeployAction {
    /// Deploy from inline code
    Code {
        /// JavaScript/TypeScript code
        code: String,
        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,
    },
    /// Deploy from a file
    File {
        /// Path to file
        path: String,
        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,
    },
    /// Deploy from a local directory
    Dir {
        /// Path to directory
        path: String,
        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,
    },
    /// Deploy from a git repo
    Repo {
        /// Repo ID or URL
        repo: String,
        /// Branch
        #[arg(long)]
        branch: Option<String>,
        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,
    },
    /// List deployments
    List,
}

// --- Git ---

#[derive(Subcommand)]
enum GitAction {
    /// Create a git repository
    Create {
        /// Repository name
        #[arg(short, long)]
        name: Option<String>,
        /// Create as public
        #[arg(long)]
        public: bool,
        /// Default branch name
        #[arg(long)]
        default_branch: Option<String>,
        /// Fork/import from URL
        #[arg(long)]
        source_url: Option<String>,
        /// Revision (branch/tag/sha) for source URL
        #[arg(long)]
        source_rev: Option<String>,
    },
    /// List repositories
    List {
        /// Maximum repositories to return
        #[arg(long, default_value = "20")]
        limit: i64,
        /// Offset cursor
        #[arg(long, default_value = "0")]
        offset: i64,
    },
    /// Delete a repository
    Delete {
        /// Repository ID
        id: Uuid,
    },
}

// --- Domains ---

#[derive(Subcommand)]
enum DomainAction {
    /// List verified domains
    List {
        #[arg(long, default_value = "50")]
        limit: i64,
        #[arg(long, default_value = "0")]
        offset: i64,
    },
    /// Start domain verification
    Verify {
        /// Domain to verify
        domain: String,
    },
    /// Complete domain verification
    Complete {
        /// Domain to complete
        #[arg(long, group = "target")]
        domain: Option<String>,
        /// Verification ID to complete
        #[arg(long, group = "target")]
        verification_id: Option<String>,
    },
    /// List pending verifications
    Verifications,
    /// Map a domain to a deployment or VM
    Map {
        /// Domain to map
        domain: String,
        /// Deployment ID target
        #[arg(long, group = "target")]
        deployment: Option<Uuid>,
        /// VM ID target
        #[arg(long, group = "target")]
        vm: Option<String>,
        /// VM port (required with --vm)
        #[arg(long)]
        vm_port: Option<i32>,
    },
    /// Remove a domain mapping
    Unmap {
        /// Domain to unmap
        domain: String,
    },
    /// List domain mappings
    Mappings {
        /// Filter by domain
        #[arg(long)]
        domain: Option<String>,
        #[arg(long, default_value = "50")]
        limit: i64,
        #[arg(long, default_value = "0")]
        offset: i64,
    },
}

// --- Cron ---

#[derive(Subcommand)]
enum CronAction {
    /// Create a cron schedule
    Create {
        /// Deployment ID
        #[arg(long)]
        deployment: Uuid,
        /// Cron expression
        #[arg(long)]
        cron: String,
        /// Timezone (default: UTC)
        #[arg(long, default_value = "UTC")]
        timezone: String,
        /// JSON payload
        #[arg(long)]
        payload: Option<String>,
        /// Path for scheduled trigger
        #[arg(long)]
        path: Option<String>,
    },
    /// List cron schedules
    List {
        /// Filter by deployment ID
        #[arg(long)]
        deployment: Option<Uuid>,
    },
    /// Enable a schedule
    Enable {
        /// Schedule ID
        id: Uuid,
    },
    /// Disable a schedule
    Disable {
        /// Schedule ID
        id: Uuid,
    },
    /// List executions for a schedule
    Executions {
        /// Schedule ID
        id: Uuid,
        #[arg(long, default_value = "20")]
        limit: i64,
        #[arg(long, default_value = "0")]
        offset: i64,
    },
    /// Get success rate over a time range
    SuccessRate {
        /// Schedule ID
        id: Uuid,
        /// Range start (ISO datetime)
        #[arg(long)]
        start: String,
        /// Range end (ISO datetime)
        #[arg(long)]
        end: String,
    },
}

// --- Entrypoint ---

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let fs = match Freestyle::from_env() {
        Ok(fs) => fs,
        Err(_) => {
            eprintln!("FREESTYLE_API_KEY not set. Get your key at https://admin.freestyle.sh");
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Command::Whoami => cmd_whoami(&fs, cli.json).await,
        Command::Vm { action } => cmd_vm(&fs, action, cli.json).await,
        Command::Deploy { action } => cmd_deploy(&fs, action, cli.json).await,
        Command::Run { code, file, env } => cmd_run(&fs, code, file, env, cli.json).await,
        Command::Git { action } => cmd_git(&fs, action, cli.json).await,
        Command::Domain { action } => cmd_domain(&fs, action, cli.json).await,
        Command::Cron { action } => cmd_cron(&fs, action, cli.json).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

type R = Result<(), Box<dyn std::error::Error>>;

fn parse_env_vars(env: &[String]) -> HashMap<String, String> {
    env.iter()
        .filter_map(|s| {
            let (k, v) = s.split_once('=')?;
            Some((k.to_string(), v.to_string()))
        })
        .collect()
}

fn json_out(val: &impl serde::Serialize) -> R {
    println!("{}", serde_json::to_string_pretty(val)?);
    Ok(())
}

// --- SSH ---

/// SSH into a VM by creating an ephemeral identity with VM permission,
/// spawning an `ssh` process, and cleaning up on exit.
async fn ssh_into_vm(fs: &Freestyle, vm_id: &str, delete: bool) -> R {
    eprintln!("Setting up SSH connection...");

    // Create ephemeral identity
    let identity = fs.client().handle_create_identity().await?.into_inner();
    let identity_id = identity.id;
    eprintln!("Created identity: {identity_id}");

    // Grant VM permission
    let body = GrantVmPermissionRequest {
        allowed_users: None,
    };
    fs.client()
        .handle_grant_vm_permission(&identity_id, vm_id, &body)
        .await?;

    // Create access token
    let token_resp = fs
        .client()
        .handle_create_git_token(&identity_id)
        .await?
        .into_inner();
    let token_id = token_resp.id;
    let token = &token_resp.token;

    let ssh_target = format!("{vm_id}:{token}@vm-ssh.freestyle.sh");
    eprintln!("Connecting to VM {vm_id}...");
    eprintln!("Command: ssh {ssh_target} -p 22\n");

    // Spawn SSH
    let status = tokio::process::Command::new("ssh")
        .arg(&ssh_target)
        .arg("-p")
        .arg("22")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await?;

    eprintln!(
        "\nSSH session ended (exit: {}).",
        status.code().unwrap_or(-1)
    );

    // Cleanup
    eprintln!("Cleaning up identity and token...");
    let _ = fs
        .client()
        .handle_revoke_git_token(&identity_id, &token_id)
        .await;
    let _ = fs.client().handle_delete_identity(&identity_id).await;
    eprintln!("Cleanup complete.");

    if delete {
        eprintln!("Deleting VM {vm_id}...");
        fs.vm(vm_id).delete().await?;
        eprintln!("VM deleted.");
    }

    Ok(())
}

// --- Whoami ---

async fn cmd_whoami(fs: &Freestyle, json: bool) -> R {
    let who = fs.client().handle_whoami().await?.into_inner();
    if json {
        return json_out(&who);
    }
    println!("Account: {}", who.account_id);
    if let Some(id) = who.identity_id {
        println!("Identity: {id}");
    }
    Ok(())
}

// --- VM ---

async fn cmd_vm(fs: &Freestyle, action: VmAction, json: bool) -> R {
    match action {
        VmAction::Create {
            name,
            mem,
            vcpus,
            disk,
            snapshot,
            domain,
            port,
            apt,
            exec,
            ssh,
            delete,
        } => {
            let mut builder = VmSpecBuilder::new()
                .mem_size_gb(mem)
                .vcpu_count(vcpus)
                .rootfs_size_gb(disk);

            if let Some(name) = name {
                builder = builder.discriminator(name);
            }
            if let Some(snap) = snapshot {
                builder = builder.snapshot_id(snap);
            }
            if !apt.is_empty() {
                builder = builder.apt_deps(apt);
            }
            if let Some(domain) = domain {
                builder = builder.domains([VmDomainConfig {
                    domain,
                    vm_port: None,
                }]);
            }
            {
                let parts: Vec<&str> = port.split(':').collect();
                let (target, external) = match parts.len() {
                    1 => (parts[0].parse::<i32>()?, 443),
                    2 => (parts[0].parse()?, parts[1].parse()?),
                    _ => return Err("port format: TARGET or TARGET:EXTERNAL".into()),
                };
                builder = builder.port(external, target);
            }

            let request = builder.build();
            if !json {
                eprintln!("Creating VM...");
            }
            let vm = fs.client().create_vm(&request).await?.into_inner();
            if json && exec.is_none() && !ssh {
                return json_out(&vm);
            }

            if !json {
                println!("VM: {}", vm.id);
                for d in &vm.domains {
                    println!("  https://{d}");
                }
            }

            if let Some(cmd) = &exec {
                let handle = fs.vm(&vm.id);
                let result = handle.exec(cmd).await?.into_inner();
                if json && !ssh {
                    return json_out(&serde_json::json!({
                        "vm": vm,
                        "exec": result,
                    }));
                }
                if let Some(stdout) = &result.stdout {
                    print!("{stdout}");
                }
                if let Some(stderr) = &result.stderr {
                    eprint!("{stderr}");
                }
            }

            if ssh {
                ssh_into_vm(fs, &vm.id, delete).await?;
            } else if delete {
                fs.vm(&vm.id).delete().await?;
                if !json {
                    eprintln!("VM deleted.");
                }
            }
        }

        VmAction::List => {
            let list = fs.client().list_vms(None).await?.into_inner();
            if json {
                return json_out(&list);
            }
            println!(
                "{} VMs ({} running, {} stopped, {} suspended)",
                list.total_count, list.running_count, list.stopped_count, list.suspended_count,
            );
            for vm in &list.vms {
                println!("  {} {:?}", vm.id, vm.state);
            }
        }

        VmAction::Get { id } => {
            let info = fs.client().get_vm(&id).await?.into_inner();
            json_out(&info)?;
        }

        VmAction::Exec { id, command } => {
            let result = fs.vm(&id).exec(&command).await?.into_inner();
            if json {
                return json_out(&result);
            }
            if let Some(stdout) = &result.stdout {
                print!("{stdout}");
            }
            if let Some(stderr) = &result.stderr {
                eprint!("{stderr}");
            }
            std::process::exit(result.status_code.unwrap_or(1));
        }

        VmAction::Start { id } => {
            fs.vm(&id).start().await?;
            if !json {
                println!("Started {id}");
            }
        }

        VmAction::Stop { id } => {
            fs.vm(&id).stop().await?;
            if !json {
                println!("Stopped {id}");
            }
        }

        VmAction::Delete { ids } => {
            for id in &ids {
                fs.vm(id).delete().await?;
                if !json {
                    println!("Deleted {id}");
                }
            }
        }

        VmAction::Ssh { id, delete } => {
            ssh_into_vm(fs, &id, delete).await?;
        }

        VmAction::Snapshot { id } => {
            let resp = fs.vm(&id).snapshot().await?.into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Snapshot: {}", resp.snapshot_id.0);
        }
    }
    Ok(())
}

// --- Deploy ---

async fn cmd_deploy(fs: &Freestyle, action: DeployAction, json: bool) -> R {
    if let DeployAction::List = action {
        let list = fs
            .client()
            .handle_list_web_deploys(50, 0, "")
            .await?
            .into_inner();
        if json {
            return json_out(&list);
        }
        for d in &list.entries {
            println!("  {} {:?}  {}", d.deployment_id, d.state, d.provisioned_at);
        }
        return Ok(());
    }

    let (source, env_vars) = match action {
        DeployAction::Code { code, env } => {
            let mut files = HashMap::new();
            files.insert(
                "index.ts".to_string(),
                FreestyleFile {
                    content: code,
                    encoding: None,
                    executable: None,
                },
            );
            (DeploymentSource::Files { files }, parse_env_vars(&env))
        }

        DeployAction::File { path, env } => {
            let code = tokio::fs::read_to_string(&path).await?;
            let mut files = HashMap::new();
            files.insert(
                "index.ts".to_string(),
                FreestyleFile {
                    content: code,
                    encoding: None,
                    executable: None,
                },
            );
            (DeploymentSource::Files { files }, parse_env_vars(&env))
        }

        DeployAction::Dir { path, env } => {
            let entries = freestyle_sandboxes::read_files(&path).await?;
            let mut files = HashMap::new();
            for entry in entries {
                files.insert(
                    entry.path,
                    FreestyleFile {
                        content: entry.content,
                        encoding: Some("base64".to_string()),
                        executable: None,
                    },
                );
            }
            (DeploymentSource::Files { files }, parse_env_vars(&env))
        }

        DeployAction::Repo { repo, branch, env } => {
            let url = if repo.contains('/') || repo.starts_with("http") {
                repo
            } else {
                format!("https://git.freestyle.sh/{repo}")
            };
            (
                DeploymentSource::Git {
                    url,
                    branch,
                    dir: None,
                },
                parse_env_vars(&env),
            )
        }

        DeployAction::List => unreachable!(),
    };

    let env_map = if env_vars.is_empty() {
        None
    } else {
        Some(env_vars)
    };

    if !json {
        eprintln!("Deploying...");
    }
    let payload = FreestyleDeployWebPayloadV2 {
        source,
        config: FreestyleDeployWebConfiguration {
            env_vars: env_map,
            ..Default::default()
        },
    };
    let resp = fs
        .client()
        .handle_deploy_web_v2(&payload)
        .await?
        .into_inner();
    let val = serde_json::to_value(&resp)?;
    if json {
        return json_out(&val);
    }
    if let Some(id) = val.get("deploymentId") {
        println!("Deployment: {}", id.as_str().unwrap_or(&id.to_string()));
    }
    if let Some(id) = val.get("projectId") {
        println!("Project:    {}", id.as_str().unwrap_or(&id.to_string()));
    }
    Ok(())
}

// --- Run ---

async fn cmd_run(
    fs: &Freestyle,
    code: Option<String>,
    file: Option<String>,
    env: Vec<String>,
    _json: bool,
) -> R {
    let script = match (code, file) {
        (Some(code), _) => code,
        (_, Some(path)) => tokio::fs::read_to_string(&path).await?,
        _ => return Err("specify --code or --file".into()),
    };

    let env_map = parse_env_vars(&env);
    let config = if env_map.is_empty() {
        None
    } else {
        Some(FreestyleExecuteScriptParamsConfiguration {
            env_vars: env_map,
            ..Default::default()
        })
    };

    let params = FreestyleExecuteScriptParams { script, config };
    let result = fs
        .client()
        .handle_execute_script_v3(&params)
        .await?
        .into_inner();
    json_out(&result)
}

// --- Git ---

async fn cmd_git(fs: &Freestyle, action: GitAction, json: bool) -> R {
    match action {
        GitAction::Create {
            name,
            public,
            default_branch,
            source_url,
            source_rev,
        } => {
            let body = HandleCreateRepoBody {
                name,
                public,
                default_branch,
                source: source_url.map(|url| CreateRepoSource {
                    url,
                    rev: source_rev,
                    all_branches: None,
                    branch: None,
                    depth: None,
                }),
                import: None,
            };
            if !json {
                eprintln!("Creating repository...");
            }
            let resp = fs.client().handle_create_repo(&body).await?.into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Repo: {}", resp.repo_id);
            println!("  https://git.freestyle.sh/{}", resp.repo_id);
        }

        GitAction::List { limit, offset } => {
            let resp = fs
                .client()
                .handle_list_repositories(Some(limit), Some(offset))
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            // RepositoryMetadata doesn't include repo IDs — dump as JSON
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }

        GitAction::Delete { id } => {
            fs.client().handle_delete_repo(&id.to_string()).await?;
            if !json {
                println!("Deleted {id}");
            }
        }
    }
    Ok(())
}

// --- Domain ---

async fn cmd_domain(fs: &Freestyle, action: DomainAction, json: bool) -> R {
    match action {
        DomainAction::List { limit, offset } => {
            let resp = fs
                .client()
                .handle_list_domains(None, Some(limit), Some(offset))
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            for d in &resp {
                println!(
                    "  {} {}",
                    d.domain,
                    if d.verified_dns { "(verified)" } else { "" }
                );
            }
        }

        DomainAction::Verify { domain } => {
            let body = FreestyleDomainVerificationRequest {
                domain: domain.clone(),
            };
            let resp = fs
                .client()
                .handle_create_domain_verification(&body)
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Add this DNS TXT record:");
            println!("  Name:  _freestyle_custom_hostname.{domain}");
            println!("  Value: {}", resp.verification_code);
            println!("\nThen run: freestyle domain complete --domain {domain}");
        }

        DomainAction::Complete {
            domain,
            verification_id,
        } => {
            let body = match (domain, verification_id) {
                (Some(d), _) => FreestyleVerifyDomainRequest::Domain(d),
                (_, Some(id)) => FreestyleVerifyDomainRequest::Id(id.parse()?),
                _ => return Err("specify --domain or --verification-id".into()),
            };
            let resp = fs.client().handle_verify_domain(&body).await?.into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Verified: {}", resp.domain);
        }

        DomainAction::Verifications => {
            let resp = fs
                .client()
                .handle_list_domain_verification_requests()
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            for v in &resp {
                println!("  {} (code: {})", v.domain, v.verification_code);
            }
        }

        DomainAction::Map {
            domain,
            deployment,
            vm,
            vm_port,
        } => {
            let body = CreateDomainMappingRequest {
                deployment_id: deployment,
                vm_id: vm.map(VmId),
                vm_port,
            };
            let resp = fs
                .client()
                .handle_insert_domain_mapping(&domain, &body)
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Mapped {domain}");
        }

        DomainAction::Unmap { domain } => {
            fs.client().handle_delete_domain_mapping(&domain).await?;
            if !json {
                println!("Unmapped {domain}");
            }
        }

        DomainAction::Mappings {
            domain,
            limit,
            offset,
        } => {
            let resp = fs
                .client()
                .handle_list_domain_mappings(domain.as_deref(), None, Some(limit), Some(offset))
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            for m in &resp {
                println!(
                    "  {} -> {}",
                    m.domain,
                    m.deployment_id
                        .as_ref()
                        .map(|id| format!("deployment:{id}"))
                        .or_else(|| m.vm_id.as_ref().map(|id| format!("vm:{id}")))
                        .unwrap_or_else(|| "-".into()),
                );
            }
        }
    }
    Ok(())
}

// --- Cron ---

async fn cmd_cron(fs: &Freestyle, action: CronAction, json: bool) -> R {
    match action {
        CronAction::Create {
            deployment,
            cron,
            timezone,
            payload,
            path,
        } => {
            let body = CreateScheduleRequest {
                cron,
                deployment_id: deployment,
                path,
                payload: payload.map(|p| serde_json::from_str(&p)).transpose()?,
                timezone: Some(timezone),
            };
            let resp = fs
                .client()
                .handle_create_schedule(&body)
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Schedule: {}", resp.schedule.id);
        }

        CronAction::List { deployment } => {
            let resp = fs
                .client()
                .handle_list_schedules(deployment.as_ref(), None, None)
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            for s in &resp.schedules {
                println!(
                    "  {} {} {} {}",
                    s.id,
                    s.cron,
                    s.timezone,
                    if s.active { "active" } else { "disabled" },
                );
            }
        }

        CronAction::Enable { id } => {
            let body = UpdateScheduleRequest {
                active: Some(true),
                cron: None,
                deployment_id: None,
                path: None,
                payload: None,
                timezone: None,
            };
            fs.client().handle_update_schedule(&id, &body).await?;
            if !json {
                println!("Enabled {id}");
            }
        }

        CronAction::Disable { id } => {
            let body = UpdateScheduleRequest {
                active: Some(false),
                cron: None,
                deployment_id: None,
                path: None,
                payload: None,
                timezone: None,
            };
            fs.client().handle_update_schedule(&id, &body).await?;
            if !json {
                println!("Disabled {id}");
            }
        }

        CronAction::Executions { id, limit, offset } => {
            let resp = fs
                .client()
                .handle_list_schedule_executions(&id, Some(limit), Some(offset))
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            for e in &resp.executions {
                println!("  {} {:?}", e.id, e.status);
            }
        }

        CronAction::SuccessRate { id, start, end } => {
            let start_dt: chrono::DateTime<chrono::Utc> = start
                .parse()
                .map_err(|e| format!("invalid start datetime: {e}"))?;
            let end_dt: chrono::DateTime<chrono::Utc> = end
                .parse()
                .map_err(|e| format!("invalid end datetime: {e}"))?;
            let resp = fs
                .client()
                .handle_get_schedule_success_rate(&id, &end_dt, &start_dt)
                .await?
                .into_inner();
            if json {
                return json_out(&resp);
            }
            println!("Schedule: {id}");
            println!("  Range:        {} → {}", resp.start, resp.end);
            println!("  Total:        {}", resp.total);
            println!("  Succeeded:    {}", resp.succeeded);
            println!("  Failed:       {}", resp.failed);
            println!("  Success rate: {}", resp.success_rate);
        }
    }
    Ok(())
}
