// CLI entry point
//
// Parse user commands
// Dispatch subommands
// Handle errors cleanly

mod ipc;

use std::env::args;
use std::process::exit;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let mut args = args().skip(1);
    match args.next().as_deref() {
        Some("sniff") => {
            let interface = args.next().unwrap_or_else(|| "eth0".to_string());
            let verbose = true;
            ipc::client::sniff(interface, verbose).await?;
        }
        Some("stop") => {
            ipc::client::stop().await?;
        }
        _ => {
            help();
            exit(0);
        }
    }
    Ok(())
}

fn help() {
    println!(
        "usage:
fw-ctl sniff [ -v ]
    Start firewall. Use -v for verbose mode.
"
    );
}
