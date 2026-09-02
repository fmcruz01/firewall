use std::net::IpAddr;

use thiserror::Error;
use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub name: String,
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
    pub fn new(ip: IpAddr, mask: u8) -> Self {
        Subnet { ip, mask }
    }
}
