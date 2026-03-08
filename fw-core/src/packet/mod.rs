pub mod ethernet;
pub mod icmp;
pub mod ipv4;
pub mod ipv6;
pub mod tcp;
pub mod udp;

pub use ethernet::EthernetHeader;
pub use ipv4::IPv4Header;
pub use ipv6::IPv6Header;
pub use tcp::TcpHeader;

use crate::packet::udp::UdpHeader;

pub struct ParsedPacket<'a> {
    pub ethernet: EthernetHeader<'a>,
    pub ip: IpHeader<'a>,
    pub transport: TransportHeader<'a>,
}

pub enum IpHeader<'a> {
    V4(IPv4Header<'a>),
    V6(IPv6Header<'a>),
}

pub enum TransportHeader<'a> {
    Tcp(TcpHeader<'a>),
    Udp(UdpHeader<'a>),
}

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
    Unknown,
}

impl<'a> IpHeader<'a> {
    pub fn protocol(&self) -> &Protocol {
        match self {
            IpHeader::V4(h) => &h.protocol,
            IpHeader::V6(h) => &h.protocol,
        }
    }
    pub fn data(&self) -> &'a [u8] {
        match self {
            IpHeader::V4(h) => h.data,
            IpHeader::V6(h) => h.data,
        }
    }
}

pub fn parse_packet<'a>(bytes: &[u8]) -> Option<ParsedPacket<'_>> {
    let eth = EthernetHeader::parse(bytes)?;
    let ip_header = match eth.ether_type {
        ethernet::EtherType::IPv4 => IpHeader::V4(IPv4Header::parse(eth.payload)?),
        ethernet::EtherType::IPv6 => IpHeader::V6(IPv6Header::parse(eth.payload)?),
        ethernet::EtherType::Unknown => return None,
    };

    let transport_header = match ip_header.protocol() {
        Protocol::TCP => TransportHeader::Tcp(TcpHeader::parse(ip_header.data())?),
        Protocol::UDP => TransportHeader::Udp(UdpHeader::parse(ip_header.data())?),
        _ => return None,
    };

    Some(ParsedPacket {
        ethernet: eth,
        ip: ip_header,
        transport: transport_header,
    })
}
impl std::fmt::Display for IpHeader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IpHeader::V4(h) => write!(f, "{}", h),
            IpHeader::V6(h) => write!(f, "{}", h),
        }
    }
}

impl std::fmt::Display for TransportHeader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransportHeader::Tcp(h) => write!(f, "{}", h),
            TransportHeader::Udp(h) => write!(f, "{}", h),
        }
    }
}
impl std::fmt::Display for ParsedPacket<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"------------Packet-------------\n{}\n{}\n{}\n", self.ethernet, self.ip, self.transport)
    }
}
