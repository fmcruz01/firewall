// Own the packet-processing loop
//
// Receive packets from kernel
// Call into fw-core::engine::pipeline
// Pass verdicts to enforcement
// Forward telemetry events
use pcap::Device;

pub(crate) fn start(verbose: bool) {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    while let Ok(packet) = cap.next_packet() {
        if verbose { println!("received packet {:?}", packet); }
    }
}
