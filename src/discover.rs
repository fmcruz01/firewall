use crate::models::Device;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::ffi::CStr;
use std::time::Duration;

pub fn scan_network(interface: String) -> Result<()> {
    progress_bar();
    let mut ifaddr_list: *mut libc::ifaddrs = std::ptr::null_mut();
    let result = unsafe { libc::getifaddrs(&mut ifaddr_list) };

    if result != 0 {
        // handle error
    } else {
        // debug existing interfaces
        let mut curr = ifaddr_list;
        while !std::ptr::eq(curr, std::ptr::null_mut()) {
            unsafe {
                let c_str = CStr::from_ptr((*curr).ifa_name);
                println!("{:?}", c_str);
                curr = (*curr).ifa_next;
            }
        }
    }

    unsafe {
        libc::freeifaddrs(ifaddr_list);
    }

    Ok(())
}

fn progress_bar() {
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Scanning network...");

    pb.finish_and_clear();
    println!("Scan complete \x1b[32m✓\x1b[0m");
}

fn output_found_device(spinner: &ProgressBar, device: &Device) {
    spinner.println(format!("{:<15} {}", device.ip, device.name));
}
