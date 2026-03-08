use std::fmt;
use super::Protocol;

#[repr(C)]
pub struct IPv6Header<'a> {
    pub dst: [u16; 8],
    pub src: [u16; 8],
    pub ttl: u8,
    pub protocol: Protocol,
    pub data: &'a [u8],
}

impl IPv6Header<'_> {
    pub fn parse(bytes: &[u8]) -> Option<IPv6Header<'_>> {
        Some(IPv6Header {
            dst: std::array::from_fn(|i| {
                let offset = 24 + i * 2;
                u16::from_be_bytes(bytes[offset..=offset + 1].try_into().unwrap())
            }),
            src: std::array::from_fn(|i| {
                let offset = 8 + i * 2;
                u16::from_be_bytes(bytes[offset..=offset + 1].try_into().unwrap())
            }),
            protocol: match bytes[6] {
                6 => Protocol::TCP,
                17 => Protocol::UDP,
                _ => Protocol::Unknown,
            },
            ttl: bytes[7],
            data: &bytes[40..],
        })
    }
}

impl fmt::Display for IPv6Header<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IPv6:\nDestination IP: {:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}\nSource IP: {:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}\nProtocol: {:?}\nTTL: {}\n",
            self.dst[0],
            self.dst[1],
            self.dst[2],
            self.dst[3],
            self.dst[4],
            self.dst[5],
            self.dst[6],
            self.dst[7],
            self.src[0],
            self.src[1],
            self.src[2],
            self.src[3],
            self.src[4],
            self.src[5],
            self.src[6],
            self.src[7],
            self.protocol,
            self.ttl
        )
    }
}
