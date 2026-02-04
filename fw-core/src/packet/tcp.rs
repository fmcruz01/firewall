#[repr(C)]
pub struct TcpHeader {}

// Parse TCP headers and expose connection-relevant fields
// Extract port, flags, sequence/ack numbers
// Validate header length
// Expose flags in a form usable by the state machine
impl TcpHeader {
    pub fn parse() -> Option<Self> {
        None
    }
}
