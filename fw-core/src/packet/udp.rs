use std::fmt;

#[repr(C)]
pub struct UdpHeader<'a> {
    pub src_port: u16,
    pub dst_port: u16,
    pub data: &'a [u8],
}

impl UdpHeader<'_> {
    pub fn parse(bytes: &[u8]) -> Option<UdpHeader<'_>> {
        Some(UdpHeader {
            src_port: u16::from_be_bytes(bytes[0..=1].try_into().ok()?),
            dst_port: u16::from_be_bytes(bytes[2..=3].try_into().ok()?),
            data: &bytes[8..],
        })
    }
}

impl fmt::Display for UdpHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UDP Header:\nSource Port: {}\nDestination Port: {}",
            self.src_port, self.dst_port
        )
    }
}
