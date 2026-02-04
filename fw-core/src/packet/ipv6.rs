#[repr(C)]
pub struct IPv6Packet {}

// IPv6 packet should parse IPv6 Headers and extension chains
// Walk extension headers in a controlled nad bounded way
// Extract next-header protocol and addresses
// Expose extension header metadata
impl IPv6Packet {
    pub fn parse() -> Option<Self> {
        None
    }
}
