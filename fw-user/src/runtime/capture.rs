use fw_core::packet::{EthernetHeader, IPv4Header, IPv6Header};
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
            let eth_head = EthernetHeader::parse(packet.data).ok_or_else(|| {
                eprintln!("Error parsing Ethernet Frame");
                RuntimeError::CaptureError
            })?;

            println!("received packet: {}", eth_head);
            match eth_head.ether_type {
                fw_core::packet::ethernet::EtherType::IPv4 => {
                    let ipv4_header = IPv4Header::parse(eth_head.payload).ok_or_else(|| {
                        eprintln!("error parsing IPv4 header");
                        RuntimeError::CaptureError
                    })?;
                    println!("IP Header: {}", ipv4_header);
                }
                fw_core::packet::ethernet::EtherType::IPv6 => {
                    let ipv6_header = IPv6Header::parse(eth_head.payload).ok_or_else(|| {
                        eprintln!("error parsing IPv6 header");
                        RuntimeError::CaptureError
                    })?;
                    println!("IP Header: {}", ipv6_header);
                }
                fw_core::packet::ethernet::EtherType::Unknown => {
                    eprintln!("Error parsing packet.");
                }
            }
        }
    }
    Ok(())
}
