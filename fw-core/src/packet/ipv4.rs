#[repr(C)]
pub struct IPv4Packet {}

// IPv4 packet should parse fixed and variable IPv4 Headers
// Validate header length, total length, checksum
// Extract protocol, src/dst addresses
// Detect fragmentation and axpose fragment metadata
impl IPv4Packet {
    pub fn parse() -> Option<Self> {
        None
    }
}
