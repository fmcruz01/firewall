use fw_core::packet::parse_packet;
use pcap::Device;

#[derive(Debug)]
pub enum RuntimeError {
    PermissionDenied,
    InterfaceNotFound,
    CaptureError,
}

pub fn start_capture(verbose: bool) -> Result<(), RuntimeError> {
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
            match parse_packet(packet.data) {
                Some(h) => {
                    println!("{}", h);
                }
                None => {
                    println!("Error parsing packet");
                }
            }
        }
    }
    Ok(())
}
