use std::fmt;

#[repr(C)]
#[derive(Debug)]
pub struct EthernetHeader<'a> {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ether_type: EtherType,
    pub payload: &'a [u8],
}
#[derive(Debug)]
pub enum EtherType {
    IPv4,
    IPv6,
    Unknown,
}

impl EthernetHeader<'_> {
    pub fn parse(bytes: &[u8]) -> Option<EthernetHeader<'_>> {
        let dst: [u8; 6] = bytes[0..=5].try_into().ok()?;
        let src: [u8; 6] = bytes[6..=11].try_into().ok()?;
        Some(EthernetHeader {
            dst,
            src,
            ether_type: match ((bytes[12] as u16) << 8) + bytes[13] as u16 {
                0x0800 => EtherType::IPv4,
                0x86DD => EtherType::IPv6,
                _ => EtherType::Unknown,
            },
            payload: &bytes[14..],
        })
    }
}

impl fmt::Display for EthernetHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EthHeader:\nDestination MAC: {:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}\nSource MAC: {:X?}:{:X?}:{:X?}:{:X?}:{:X?}:{:X?}\nEthernet Type: {:?}\n",
            self.dst[0],
            self.dst[1],
            self.dst[2],
            self.dst[3],
            self.dst[4],
            self.dst[5],
            self.src[0],
            self.src[1],
            self.src[2],
            self.src[3],
            self.src[4],
            self.src[5],
            self.ether_type
        )
    }
}
