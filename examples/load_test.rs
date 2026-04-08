//! Deploy a function and hit it concurrently to load-test.
//!
//! Shows: serverless deployment from inline code, deployment fetch proxy,
//! and concurrent async workflows.
//!
//! ```sh
//! export FREESTYLE_API_KEY="your-key"
//! cargo run --example load_test -- 50
//! ```

use freestyle_sandboxes::Freestyle;
use freestyle_sandboxes::types::*;
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let concurrency: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let fs = Freestyle::from_env()?;

    // Deploy a simple function
    println!("Deploying test function...");
    let mut files = HashMap::new();
    files.insert(
        "index.ts".to_string(),
        FreestyleFile {
            content: r#"export default () => new Response(JSON.stringify({ ok: true, ts: Date.now() }), { headers: { "content-type": "application/json" } })"#.to_string(),
            encoding: None,
            executable: None,
        },
    );

    let payload = FreestyleDeployWebPayloadV2 {
        source: DeploymentSource::Files { files },
        config: FreestyleDeployWebConfiguration::default(),
    };

    let resp = fs.client().handle_deploy_web_v2(&payload).await?;
    let json = serde_json::to_value(resp.into_inner())?;
    let deployment_id: uuid::Uuid = json
        .get("deploymentId")
        .and_then(|v| v.as_str())
        .expect("missing deploymentId")
        .parse()?;

    println!("Deployed: {deployment_id}");
    let handle = fs.deployment(deployment_id);

    // Warm up
    println!("Warming up...");
    let _ = handle.fetch(reqwest::Method::GET, "/", None).await?;

    // Concurrent load test
    println!("Sending {concurrency} concurrent requests...");
    let start = Instant::now();

    let mut tasks = Vec::new();
    for i in 0..concurrency {
        let h = fs.deployment(deployment_id);
        tasks.push(tokio::spawn(async move {
            let t = Instant::now();
            let result = h.fetch(reqwest::Method::GET, "/", None).await;
            let elapsed = t.elapsed();
            match result {
                Ok(resp) => (i, resp.status().as_u16(), elapsed),
                Err(e) => {
                    eprintln!("  req {i}: {e}");
                    (i, 0, elapsed)
                }
            }
        }));
    }

    let mut results = Vec::new();
    for task in tasks {
        results.push(task.await?);
    }

    let total = start.elapsed();
    let successes = results.iter().filter(|(_, s, _)| *s == 200).count();
    let avg_ms = results.iter().map(|(_, _, d)| d.as_millis()).sum::<u128>() / concurrency as u128;
    let p99_ms = {
        let mut durations: Vec<_> = results.iter().map(|(_, _, d)| d.as_millis()).collect();
        durations.sort();
        durations[durations.len() * 99 / 100]
    };

    println!("\nResults:");
    println!("  Total time:  {:.1}ms", total.as_secs_f64() * 1000.0);
    println!("  Successes:   {successes}/{concurrency}");
    println!("  Avg latency: {avg_ms}ms");
    println!("  P99 latency: {p99_ms}ms");
    println!(
        "  RPS:         {:.0}",
        concurrency as f64 / total.as_secs_f64()
    );

    Ok(())
}
