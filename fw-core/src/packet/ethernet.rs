#[repr(C)]
pub struct EthernetHeader {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ether_type: u16,
}

impl EthernetHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        // TODO: Implement parse method. What is bytes? What is the order? How should they be
        // processsed?
        // Validate minimum frame length
        // Should payload be passed to IP parsing?
        None
    }
}
