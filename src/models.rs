use std::net::IpAddr;

use thiserror::Error;
use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub name: String,
}

#[derive(Debug)]
pub struct NetworkInterface {
    pub name: String,
    pub addresses: Vec<IpAddr>,
}

#[derive(Debug, Error)]
pub enum DiscoverError {
    #[error("failed to connect to network interface")]
    SocketError {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to find {iface} network interface")]
    NetworkInterfaceNotFound {
        iface: String,
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

impl NetworkInterface {
    pub fn new() -> Self {
        NetworkInterface {
            name: String::new(),
            addresses: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    pub fn add_addr(&mut self, addr: IpAddr) {
        self.addresses.push(addr);
    }
}
