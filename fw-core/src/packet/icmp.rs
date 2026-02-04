#[repr(C)]
pub struct IcmpHeader {

}

// Parse ICMP v4 and v6 headers
// PArse ICMP type and code
// Extract minimal metadata required for rules
// Identify error vs informational messages
impl IcmpHeader {
    pub fn parse() -> Option<Self> {
        None
    }
}
