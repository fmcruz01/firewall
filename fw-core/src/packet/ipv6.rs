use std::fmt;

#[repr(C)]
pub struct IPv6Header<'a> {
    pub dst: [u8; 16],
    pub src: [u8; 16],
    pub ttl: u8,
    pub protocol: Protocol,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
    Unknown,
}

// IPv4 packet should parse fixed and variable IPv4 Headers
// Validate header length, total length, checksum
// Extract protocol, src/dst addresses
// Detect fragmentation and axpose fragment metadata
impl IPv6Header<'_> {
    pub fn parse(bytes: &[u8]) -> Option<IPv6Header<'_>> {
        let src = bytes[8..=23].try_into().ok()?;
        let dst = bytes[24..=39].try_into().ok()?;
        Some(IPv6Header {
            dst,
            src,
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

