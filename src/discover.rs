use crate::models::Device;
use crate::route_table::get_route_table;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn scan_network(interface: String) -> Result<()> {
    let pb = progress_bar();
    if &interface == "default" {
        let iface = get_route_table().with_context(|| format!("failed to get routing table"))?;
        println!("{iface:?}");
    }
    end_progress_bar(pb);
    Ok(())
}

fn progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Scanning network...");
    pb
}

fn end_progress_bar(pb: ProgressBar) {
    pb.finish_and_clear();
    println!("Scan complete \x1b[32m✓\x1b[0m");
}

fn output_found_device(spinner: &ProgressBar, device: &Device) {
    spinner.println(format!("{:<15} {}", device.ip, device.name));
}
