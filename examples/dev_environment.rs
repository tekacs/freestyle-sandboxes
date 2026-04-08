//! Spin up a development VM from a git repo, install deps, start a dev server,
//! and wait until it's reachable.
//!
//! Shows: VmSpecBuilder composition, run_commands chaining, wait_for, git repo
//! mounting, and using the VM handle for exec after creation.
//!
//! ```sh
//! export FREESTYLE_API_KEY="your-key"
//! cargo run --example dev_environment -- <repo-id>
//! ```

use freestyle_sandboxes::{Freestyle, VmSpecBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_id = std::env::args()
        .nth(1)
        .expect("usage: dev_environment <repo-id>");

    let fs = Freestyle::from_env()?;

    // Build a VM spec: clone a repo, install deps, start a dev server
    let request = VmSpecBuilder::new()
        .mem_size_gb(2)
        .vcpu_count(2)
        .apt_deps(["curl", "git"])
        .repo(&repo_id, "/workspace")
        .workdir("/workspace")
        .run_commands(["npm install", "npm run build"])
        .wait_for("curl -sf http://localhost:3000 > /dev/null", 2)
        .port(443, 3000)
        .build();

    println!("Creating VM...");
    let resp = fs.client().create_vm(&request).await?;
    let vm_resp = resp.into_inner();
    println!("VM created: {}", vm_resp.id);
    for domain in &vm_resp.domains {
        println!("  https://{domain}");
    }

    // Use the VM handle for further interaction
    let vm = fs.vm(&vm_resp.id);

    // Check what's running
    let exec_result = vm.exec("ps aux | head -20").await?;
    let inner = exec_result.into_inner();
    println!("\nProcesses:\n{}", inner.stdout.as_deref().unwrap_or(""));

    // Read a file from the workspace
    let readme = vm.fs().read_text_file("/workspace/README.md").await;
    match readme {
        Ok(content) => println!(
            "README.md ({} bytes):\n{}",
            content.len(),
            &content[..content.len().min(200)]
        ),
        Err(e) => println!("No README.md: {e}"),
    }

    println!("\nVM is running. Delete with: freestyle vm delete {}", vm_resp.id);
    Ok(())
}
