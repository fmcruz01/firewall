// Own the packet-processing loop
//
// Receive packets from kernel
// Call into fw-core::engine::pipeline
// Pass verdicts to enforcement
// Forward telemetry events
use crate::RuntimeError;
use chrono::DateTime;
use pcap::Device;
//use signal_hook::consts::{SIGTERM, SIGINT};

pub(crate) fn start_capture(verbose: bool) -> Result<(), RuntimeError> {
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

    while let Ok(packet) = cap.next_packet() {
        if verbose {
            println!(
                "received packet at: {:?}",
                DateTime::from_timestamp_secs(packet.header.ts.tv_sec).unwrap()
            );
        }
    }
    Ok(())
}
