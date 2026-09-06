use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use thiserror::Error;
use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub device_type: String,
}

#[repr(C)]
pub struct icmphdr {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub id: u16,
    pub seq: u16,
}

#[derive(Debug, PartialEq)]
pub struct Subnet {
    ip: IpAddr,
    mask: u8,
}

#[derive(Debug, PartialEq)]
pub struct NetworkInterface {
    name: String,
    subnets: Vec<Subnet>,
}

#[derive(Debug, Error)]
pub enum DiscoverError {
    #[error("failed to connect to network interface")]
    SocketError {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to find {iface} network interface")]
    NetworkInterfaceNotFound { iface: String },
}

impl NetworkInterface {
    pub fn new() -> Self {
        NetworkInterface {
            name: String::new(),
            subnets: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_subnet(&mut self, subnet: Subnet) {
        if !self.subnets.contains(&subnet) {
            self.subnets.push(subnet);
        }
    }

    pub fn get_subnets(&self) -> &Vec<Subnet> {
        &self.subnets
    }
}

impl Subnet {
    pub fn new(addr: IpAddr, mask: u8) -> Self {
        match addr {
            IpAddr::V4(addr_v4) => {
                let mut bitmask = 1;
                for _ in 0..mask {
                    bitmask = (bitmask << 1) + 1;
                }

                for _ in 0..32 - mask {
                    bitmask = bitmask << 1;
                }
                let ip = addr_v4.to_bits() & bitmask;
                Subnet {
                    ip: IpAddr::V4(Ipv4Addr::from_bits(ip)),
                    mask,
                }
            }
            IpAddr::V6(addr_v6) => {
                let mut bitmask: u128 = 1;
                for _ in 0..mask {
                    bitmask = (bitmask << 1) + 1;
                }

                for _ in 0..64 - mask {
                    bitmask = bitmask << 1;
                }
                let ip = addr_v6.to_bits() & bitmask;
                Subnet {
                    ip: IpAddr::V6(Ipv6Addr::from_bits(ip)),
                    mask,
                }
            }
        }
    }
}

impl Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ip, self.mask)
    }
}

impl Display for NetworkInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Subnets:", self.name)?;
        for subnet in &self.subnets {
            write!(f, " {}", subnet)?;
        }
        Ok(())
    }
}
