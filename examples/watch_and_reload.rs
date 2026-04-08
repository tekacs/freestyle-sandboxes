//! Watch a VM's filesystem for changes and re-run a command on each change.
//!
//! Shows: FileChangeStream (async Stream), VM exec, and reactive workflows
//! that aren't possible with a CLI alone.
//!
//! ```sh
//! export FREESTYLE_API_KEY="your-key"
//! cargo run --example watch_and_reload -- <vm-id> "npm test"
//! ```

use freestyle_sandboxes::Freestyle;
use futures_core::Stream;
use std::pin::Pin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: watch_and_reload <vm-id> <command>");
        std::process::exit(1);
    }
    let vm_id: uuid::Uuid = args[1].parse()?;
    let command = &args[2];

    let fs = Freestyle::from_env()?;
    let vm = fs.vm(vm_id);

    println!("Watching VM {vm_id} for file changes...");
    println!("Will run `{command}` on each change.\n");

    let mut stream: Pin<Box<dyn Stream<Item = _>>> = Box::pin(vm.watch_files().await?);

    loop {
        let event = std::future::poll_fn(|cx| Pin::as_mut(&mut stream).poll_next(cx)).await;
        match event {
            Some(Ok(change)) => {
                println!("File changed: {}", change.0);
                println!("Running `{command}`...");
                match vm.exec(command).await {
                    Ok(result) => {
                        let inner = result.into_inner();
                        if let Some(stdout) = &inner.stdout {
                            print!("{stdout}");
                        }
                        if let Some(stderr) = &inner.stderr {
                            eprint!("{stderr}");
                        }
                        println!("Exit: {}", inner.status_code.unwrap_or(-1));
                    }
                    Err(e) => eprintln!("Exec failed: {e}"),
                }
                println!("---");
            }
            Some(Err(e)) => {
                eprintln!("Stream error: {e}");
                break;
            }
            None => {
                println!("Watch stream ended.");
                break;
            }
        }
    }

    Ok(())
}
