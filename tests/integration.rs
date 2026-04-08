//! Integration tests for freestyle-sandboxes.
//!
//! These tests hit the live Freestyle API and require FREESTYLE_API_KEY.
//! Run with: `cargo test -- --ignored`
//!
//! Tests clean up after themselves. Each test explicitly deletes resources
//! at the end. Drop guards provide fallback cleanup if assertions panic.

use freestyle_sandboxes::types::*;
use freestyle_sandboxes::{Freestyle, VmSpecBuilder};
use std::collections::HashMap;

fn client() -> Freestyle {
    Freestyle::from_env().expect("FREESTYLE_API_KEY must be set for integration tests")
}

// ---------------------------------------------------------------------------
// Cleanup: explicit async delete + Drop fallback for panics
// ---------------------------------------------------------------------------

async fn delete_vm(fs: &Freestyle, id: &str) {
    let _ = fs.client().delete_vm(id).await;
}

async fn delete_repo(fs: &Freestyle, id: uuid::Uuid) {
    let _ = fs.client().handle_delete_repo(&id.to_string()).await;
}

/// Panic-only fallback. Spawns a thread with its own runtime since we can't
/// use the tokio runtime from inside a Drop during a panic unwind.
fn delete_vm_sync(client: freestyle_sandboxes::Client, id: String) {
    let _ = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(client.delete_vm(&id)).ok();
    })
    .join();
}

fn delete_repo_sync(client: freestyle_sandboxes::Client, id: String) {
    let _ = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(client.handle_delete_repo(&id)).ok();
    })
    .join();
}

struct VmGuard {
    client: freestyle_sandboxes::Client,
    id: String,
    cleaned: bool,
}

impl VmGuard {
    fn new(fs: &Freestyle, id: impl Into<String>) -> Self {
        Self {
            client: fs.client().clone(),
            id: id.into(),
            cleaned: false,
        }
    }
    fn defuse(&mut self) {
        self.cleaned = true;
    }
}

impl Drop for VmGuard {
    fn drop(&mut self) {
        if !self.cleaned {
            delete_vm_sync(self.client.clone(), self.id.clone());
        }
    }
}

struct RepoGuard {
    client: freestyle_sandboxes::Client,
    id: uuid::Uuid,
    cleaned: bool,
}

impl RepoGuard {
    fn new(fs: &Freestyle, id: uuid::Uuid) -> Self {
        Self {
            client: fs.client().clone(),
            id,
            cleaned: false,
        }
    }
    fn defuse(&mut self) {
        self.cleaned = true;
    }
}

impl Drop for RepoGuard {
    fn drop(&mut self) {
        if !self.cleaned {
            delete_repo_sync(self.client.clone(), self.id.to_string());
        }
    }
}

// ---------------------------------------------------------------------------
// VM lifecycle: create → exec → delete
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn vm_create_exec_delete() {
    let fs = client();

    let req = VmSpecBuilder::new()
        .mem_size_gb(1)
        .vcpu_count(1)
        .rootfs_size_gb(2)
        .build();

    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);

    let result = vm
        .exec("echo hello-from-rust")
        .await
        .expect("exec")
        .into_inner();
    assert_eq!(result.status_code, Some(0));
    assert!(
        result
            .stdout
            .as_deref()
            .unwrap_or("")
            .contains("hello-from-rust"),
        "stdout was: {:?}",
        result.stdout
    );

    let fail = vm.exec("exit 42").await.expect("exec fail").into_inner();
    assert_eq!(fail.status_code, Some(42));

    delete_vm(&fs, &vm_resp.id).await;
    guard.defuse();
}

// ---------------------------------------------------------------------------
// Filesystem: write file, read it back, stat, mkdir, exists, remove
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn vm_filesystem_operations() {
    let fs = client();

    let req = VmSpecBuilder::new().mem_size_gb(1).vcpu_count(1).build();
    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);
    let vfs = vm.fs();

    let content = "Hello from Rust integration test! 🦀\nLine two.";
    vfs.write_text_file("/tmp/test.txt", content)
        .await
        .expect("write");
    let read_back = vfs.read_text_file("/tmp/test.txt").await.expect("read");
    assert_eq!(read_back, content);

    let binary: Vec<u8> = (0..=255).collect();
    vfs.write_file("/tmp/test.bin", &binary)
        .await
        .expect("write bin");
    let read_bin = vfs.read_file("/tmp/test.bin").await.expect("read bin");
    assert_eq!(read_bin, binary);

    let stat = vfs.stat("/tmp/test.txt").await.expect("stat");
    assert!(stat.is_file);
    assert!(!stat.is_directory);
    assert_eq!(stat.size, content.len() as u64);

    assert!(!vfs.exists("/tmp/nested/dir").await.expect("exists false"));
    vfs.mkdir("/tmp/nested/dir", true).await.expect("mkdir -p");
    assert!(vfs.exists("/tmp/nested/dir").await.expect("exists true"));

    let dir_stat = vfs.stat("/tmp/nested/dir").await.expect("stat dir");
    assert!(dir_stat.is_directory);

    vfs.write_text_file("/tmp/nested/dir/a.txt", "a")
        .await
        .expect("write a");
    vfs.write_text_file("/tmp/nested/dir/b.txt", "b")
        .await
        .expect("write b");
    let listing = vfs.read_dir("/tmp/nested/dir").await.expect("read_dir");
    let names: Vec<&str> = listing.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"a.txt"), "listing: {names:?}");
    assert!(names.contains(&"b.txt"), "listing: {names:?}");

    vfs.remove("/tmp/nested", true).await.expect("remove");
    assert!(!vfs.exists("/tmp/nested").await.expect("exists after rm"));

    delete_vm(&fs, &vm_resp.id).await;
    guard.defuse();
}

// ---------------------------------------------------------------------------
// Builder: run_commands chains, apt_deps
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn vm_builder_run_commands() {
    let fs = client();

    let req = VmSpecBuilder::new()
        .mem_size_gb(1)
        .vcpu_count(1)
        .run_commands([
            "echo step1 > /tmp/build.log",
            "echo step2 >> /tmp/build.log",
            "echo step3 >> /tmp/build.log",
        ])
        .build();

    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);

    let result = vm
        .exec("cat /tmp/build.log")
        .await
        .expect("exec")
        .into_inner();
    let stdout = result.stdout.unwrap_or_default();
    assert_eq!(
        stdout.trim(),
        "step1\nstep2\nstep3",
        "out of order: {stdout}"
    );

    delete_vm(&fs, &vm_resp.id).await;
    guard.defuse();
}

#[tokio::test]
#[ignore]
async fn vm_builder_apt_deps() {
    let fs = client();

    let req = VmSpecBuilder::new()
        .mem_size_gb(1)
        .vcpu_count(1)
        .apt_deps(["jq"])
        .build();

    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);

    let result = vm
        .exec("echo '{\"a\":1}' | jq .a")
        .await
        .expect("exec jq")
        .into_inner();
    assert_eq!(result.status_code, Some(0));
    assert_eq!(result.stdout.unwrap_or_default().trim(), "1");

    delete_vm(&fs, &vm_resp.id).await;
    guard.defuse();
}

// ---------------------------------------------------------------------------
// Systemd: create a service, check status, stop it
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn vm_systemd_service_lifecycle() {
    let fs = client();

    let req = VmSpecBuilder::new().mem_size_gb(1).vcpu_count(1).build();
    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);

    vm.fs()
        .write_text_file(
            "/tmp/tick.sh",
            "#!/bin/bash\nwhile true; do echo tick >> /tmp/svc.log; sleep 1; done\n",
        )
        .await
        .expect("write script");
    vm.exec("chmod +x /tmp/tick.sh").await.expect("chmod");

    let spec = SystemdUnitSpec {
        name: "test-svc".to_string(),
        mode: SystemdUnitMode::Service,
        exec: vec!["/tmp/tick.sh".into()],
        enable: Some(true),
        after: None,
        delete_after_success: None,
        env: None,
        group: None,
        on_failure: None,
        ready_signal: None,
        remain_after_exit: None,
        requires: None,
        restart_policy: None,
        timeout_sec: None,
        user: None,
        wanted_by: None,
        watchdog_sec: None,
        workdir: None,
    };

    fs.client()
        .create_service(&vm_resp.id, &spec)
        .await
        .expect("create_service");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let svc = vm.systemd_service("test-svc");
    let status = svc.status().await.expect("status").into_inner();
    assert!(
        status.active_state == "active" || status.active_state == "activating",
        "service state: {}",
        status.active_state
    );

    let result = vm
        .exec("wc -l < /tmp/svc.log")
        .await
        .expect("wc")
        .into_inner();
    let lines: i32 = result
        .stdout
        .unwrap_or_default()
        .trim()
        .parse()
        .unwrap_or(0);
    assert!(lines >= 2, "expected at least 2 ticks, got {lines}");

    svc.stop().await.expect("stop");
    svc.delete().await.expect("delete service");

    delete_vm(&fs, &vm_resp.id).await;
    guard.defuse();
}

// ---------------------------------------------------------------------------
// Serverless: deploy code, verify it was created
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn deploy_serverless() {
    let fs = client();

    let mut files = HashMap::new();
    files.insert(
        "index.ts".to_string(),
        FreestyleFile {
            content: r#"export default () => new Response(JSON.stringify({ rust: true }))"#
                .to_string(),
            encoding: None,
            executable: None,
        },
    );

    let payload = FreestyleDeployWebPayloadV2 {
        source: DeploymentSource::Files { files },
        config: FreestyleDeployWebConfiguration::default(),
    };

    let resp = fs
        .client()
        .handle_deploy_web_v2(&payload)
        .await
        .expect("deploy")
        .into_inner();
    let json = serde_json::to_value(&resp).unwrap();
    let deployment_id = json["deploymentId"].as_str().expect("missing deploymentId");
    assert!(!deployment_id.is_empty());
    let _: uuid::Uuid = deployment_id.parse().expect("should be UUID");
}

// ---------------------------------------------------------------------------
// Git: create repo, commit files, read them back, delete
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn git_repo_lifecycle() {
    let fs = client();

    let body = HandleCreateRepoBody {
        name: Some("rust-integration-test".into()),
        public: false,
        default_branch: None,
        source: None,
        import: None,
    };
    let repo = fs
        .client()
        .handle_create_repo(&body)
        .await
        .expect("create repo")
        .into_inner();
    let mut guard = RepoGuard::new(&fs, repo.repo_id);

    let commit_body = CreateCommitRequest {
        message: "test commit from rust".to_string(),
        branch: Some("main".into()),
        files: vec![
            FileChange {
                path: "hello.txt".into(),
                content: Some("Hello from Rust!".into()),
                encoding: None,
                deleted: None,
            },
            FileChange {
                path: "data.json".into(),
                content: Some(r#"{"test": true}"#.into()),
                encoding: None,
                deleted: None,
            },
        ],
        author: Some(CommitAuthorInfo {
            name: "Rust Test".into(),
            email: "test@example.com".into(),
        }),
        expected_sha: None,
    };
    let commit = fs
        .client()
        .handle_create_commit(&repo.repo_id, &commit_body)
        .await
        .expect("create commit")
        .into_inner();
    assert!(!commit.commit.sha.is_empty());

    let contents = fs
        .client()
        .handle_get_contents(&repo.repo_id, "hello.txt", None)
        .await
        .expect("get contents")
        .into_inner();
    let content_json = serde_json::to_value(&contents).unwrap();
    assert!(content_json["content"].is_string() || content_json["name"].is_string());

    let branches = fs
        .client()
        .handle_list_branches(&repo.repo_id)
        .await
        .expect("list branches")
        .into_inner();
    let branch_json = serde_json::to_value(&branches).unwrap();
    assert!(branch_json.to_string().contains("main"));

    delete_repo(&fs, repo.repo_id).await;
    guard.defuse();
}

// ---------------------------------------------------------------------------
// VM + Git: create repo, mount it in a VM, verify files
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn vm_with_git_repo() {
    let fs = client();

    let body = HandleCreateRepoBody {
        name: Some("rust-vm-git-test".into()),
        public: false,
        default_branch: None,
        source: None,
        import: None,
    };
    let repo = fs
        .client()
        .handle_create_repo(&body)
        .await
        .expect("create repo")
        .into_inner();
    let mut repo_guard = RepoGuard::new(&fs, repo.repo_id);

    let commit_body = CreateCommitRequest {
        message: "initial".to_string(),
        branch: Some("main".into()),
        files: vec![FileChange {
            path: "README.md".into(),
            content: Some("# Test Repo\nCreated by Rust integration test.".into()),
            encoding: None,
            deleted: None,
        }],
        author: Some(CommitAuthorInfo {
            name: "Rust Test".into(),
            email: "test@example.com".into(),
        }),
        expected_sha: None,
    };
    fs.client()
        .handle_create_commit(&repo.repo_id, &commit_body)
        .await
        .expect("commit");

    let req = VmSpecBuilder::new()
        .mem_size_gb(1)
        .vcpu_count(1)
        .repo(repo.repo_id.to_string(), "/workspace")
        .workdir("/workspace")
        .build();
    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut vm_guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);

    let result = vm
        .exec("cat /workspace/README.md")
        .await
        .expect("cat")
        .into_inner();
    assert_eq!(result.status_code, Some(0));
    assert!(
        result.stdout.as_deref().unwrap_or("").contains("Test Repo"),
        "stdout: {:?}",
        result.stdout
    );

    vm.fs()
        .write_text_file("/workspace/new-file.txt", "created inside VM")
        .await
        .expect("write");
    let readback = vm
        .fs()
        .read_text_file("/workspace/new-file.txt")
        .await
        .expect("read");
    assert_eq!(readback, "created inside VM");

    delete_vm(&fs, &vm_resp.id).await;
    vm_guard.defuse();
    delete_repo(&fs, repo.repo_id).await;
    repo_guard.defuse();
}

// ---------------------------------------------------------------------------
// Exec timeout
// ---------------------------------------------------------------------------

#[tokio::test]
#[ignore]
async fn vm_exec_timeout() {
    let fs = client();

    let req = VmSpecBuilder::new().mem_size_gb(1).vcpu_count(1).build();
    let vm_resp = fs
        .client()
        .create_vm(&req)
        .await
        .expect("create_vm")
        .into_inner();
    let mut guard = VmGuard::new(&fs, &vm_resp.id);
    let vm = fs.vm(&vm_resp.id);

    let fast = vm
        .exec_with_timeout("echo quick", 5000)
        .await
        .expect("fast exec")
        .into_inner();
    assert_eq!(fast.status_code, Some(0));

    delete_vm(&fs, &vm_resp.id).await;
    guard.defuse();
}
