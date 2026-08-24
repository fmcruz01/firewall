use crate::models::{Device, DiscoverError};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use libc::{
    AF_NETLINK, NETLINK_ROUTE, SOCK_RAW, bind, close, getpid, send, sockaddr, sockaddr_nl, socket,
    socklen_t,
};
use std::{array, mem, os::fd::RawFd, time::Duration};

pub fn scan_network(interface: String) -> Result<()> {
    progress_bar();
    if &interface == "default" {
        get_route_table().with_context(|| format!("failed to get routing table"))?;
    }
    Ok(())
}

fn get_route_table() -> Result<(), DiscoverError> {
    unsafe {
        let sockfd_nl: RawFd = open_nl_socket()?;
        bind_nl_socket(sockfd_nl)?;
        close(sockfd_nl);
    }
    Ok(())
}

fn build_rtm_getroute() -> Result<(), DiscoverError> {
    Ok(())
}

fn open_nl_socket() -> Result<RawFd, DiscoverError> {
    unsafe {
        let sockfd_nl: RawFd = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
        if sockfd_nl < 0 {
            close(sockfd_nl);
            return Err(DiscoverError::SocketError {
                operation: String::from("open"),
                socket_type: String::from("netlink"),
                source: std::io::Error::last_os_error(),
            });
        }

        Ok(sockfd_nl)
    }
}

fn bind_nl_socket(sockfd_nl: RawFd) -> Result<(), DiscoverError> {
    unsafe {
        let mut saddr: sockaddr_nl = mem::zeroed();
        saddr.nl_pid = getpid() as u32;
        saddr.nl_family = AF_NETLINK as u16;
        saddr.nl_groups = 0;

        if bind(
            sockfd_nl,
            &saddr as *const _ as *const sockaddr,
            mem::size_of::<sockaddr_nl>() as socklen_t,
        ) < 0
        {
            close(sockfd_nl);
            return Err(DiscoverError::SocketError {
                operation: String::from("bind"),
                socket_type: String::from("netlink"),
                source: std::io::Error::last_os_error(),
            });
        }
        Ok(())
    }
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
