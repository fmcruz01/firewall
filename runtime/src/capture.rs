use core::packet::parse_packet;
use pcap::Device;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub enum RuntimeError {
    PermissionDenied,
    InterfaceNotFound,
    CaptureError,
}

pub fn start_capture(verbose: bool, running_flag: Arc<AtomicBool>) -> Result<(), RuntimeError> {
    let device = Device::lookup()
        .map_err(|_| RuntimeError::PermissionDenied)?
        .ok_or(RuntimeError::InterfaceNotFound)?;

    let mut cap = device.open().map_err(|err| {
        if err.to_string().contains("CAP_NET_RAW") {
            RuntimeError::PermissionDenied
        } else {
            RuntimeError::CaptureError
        }

    })?;

    while running_flag.load(Ordering::SeqCst) && let Ok(packet) = cap.next_packet()  {
        if verbose {
            match parse_packet(packet.data) {
                Ok(h) => {
                    println!("{}", h);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
    Ok(())
}
