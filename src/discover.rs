use crate::models::{Device, Subnet};
use crate::network::{get_netw_addr, ping_local_ip};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn scan_network(interface: String) -> Result<()> {
    let pb = progress_bar();
    pb.println(format!("{:<20} Device Type", "IP address"));
    if &interface == "default" {
        let ifaces = get_netw_addr().context("failed to get routing table")?;
        for iface in ifaces {
            let subnet: &Subnet = &iface.1.get_subnets()[0];
            for ip in subnet.get_subnet_ips() {
                let res = ping_local_ip(ip).with_context(|| format!("failed to ping ip {}", ip))?;
                if let Some(recv_ip) = res {
                    let device = Device {
                        ip: recv_ip.to_string(),
                        device_type: String::from("unknown"),
                    };
                    output_found_device(&pb, &device);
                }
            }
        }
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
    spinner.println(format!("{:<20} {}", device.ip, device.device_type));
}
