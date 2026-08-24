use thiserror::Error;
use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub name: String,
}

#[derive(Debug, Error)]
pub enum DiscoverError {
    #[error("failed to connect to network interface")]
    SocketError {
        #[source]
        source: std::io::Error,
    },
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RtMsg {
    pub rtm_family: u8,
    pub rtm_dst_len: u8,
    pub rtm_src_len: u8,
    pub rtm_tos: u8,

    pub rtm_table: u8,
    pub rtm_protocol: u8,
    pub rtm_scope: u8,
    pub rtm_type: u8,

    pub rtm_flags: u32,
}
