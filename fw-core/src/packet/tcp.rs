use std::fmt;

#[repr(C)]
pub struct TcpHeader<'a> {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data: &'a [u8],
}

impl TcpHeader<'_> {
    pub fn parse(bytes: &[u8]) -> Option<TcpHeader<'_>> {
        Some(TcpHeader {
            src_port: u16::from_be_bytes(bytes[0..=1].try_into().ok()?),
            dst_port: u16::from_be_bytes(bytes[2..=3].try_into().ok()?),
            seq_num: u32::from_be_bytes(bytes[4..=7].try_into().ok()?),
            ack_num: u32::from_be_bytes(bytes[8..=11].try_into().ok()?),
            data: &bytes[20..],
        })
    }
}

impl fmt::Display for TcpHeader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TCP Header:\nSource Port: {}\nDestination Port: {}\nSequence Num: {}\nAck Num: {}",
            self.src_port, self.dst_port, self.seq_num, self.ack_num
        )
    }
}
