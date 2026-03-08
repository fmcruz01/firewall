use std::fmt;
use super::Protocol;

#[repr(C)]
pub struct IPv4Header<'a> {
    pub dst: [u8; 4],
    pub src: [u8; 4],
    pub ttl: u8,
    pub protocol: Protocol,
    pub data: &'a [u8],
}

impl IPv4Header<'_> {
    pub fn parse(bytes: &[u8]) -> Option<IPv4Header<'_>> {
        Some(IPv4Header {
            dst: bytes[16..=19].try_into().ok()?,
            src: bytes[12..=15].try_into().ok()?,
            protocol: match bytes[9] {
                6 => Protocol::TCP,
                17 => Protocol::UDP,
                _ => Protocol::Unknown
            },
            ttl: bytes[8],
            data: &bytes[24..],
        })
    }
}

impl fmt::Display for IPv4Header<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IPv4:\nDestination IP: {}.{}.{}.{}\nSource IP: {}.{}.{}.{}\nProtocol: {:?}\nTTL: {}\n",
            self.dst[0],
            self.dst[1],
            self.dst[2],
            self.dst[3],
            self.src[0],
            self.src[1],
            self.src[2],
            self.src[3],
            self.protocol,
            self.ttl
        )
    }
}
