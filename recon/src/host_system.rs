use std::net::IpAddr;
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct HostSystem {
    cpu_vendor: Option<String>,
    ip_address: Option<IpAddr>,
    ip_prefix: Option<u8>,
    network_count: Option<u8>,
}

pub struct HostSystemBuilder {
    cpu_vendor: Option<String>,
    ip_address: Option<IpAddr>,
    ip_prefix: Option<u8>,
    network_count: Option<u8>,
}

impl HostSystemBuilder {
    pub fn new() -> Self {
        HostSystemBuilder {
            cpu_vendor: None,
            ip_address: None,
            ip_prefix: None,
            network_count: None,
        }
    }

    pub fn cpu_vendor(&mut self, cpu_vendor: &str) -> &mut Self {
        self.cpu_vendor = Some(cpu_vendor.to_string());
        self
    }

    pub fn ip_address(&mut self, ip_address: IpAddr) -> &mut Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn ip_prefix(&mut self, ip_prefix: u8) -> &mut Self {
        self.ip_prefix = Some(ip_prefix);
        self
    }

    pub fn network_count(&mut self, network_count: u8) -> &mut Self {
        self.network_count = Some(network_count);
        self
    }

    pub fn build(self) -> HostSystem {
        HostSystem {
            cpu_vendor: self.cpu_vendor,
            ip_address: self.ip_address,
            ip_prefix: self.ip_prefix,
            network_count: self.network_count,
        }
    }
}



impl fmt::Display for HostSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU Vendor: {}\nIP Address: {:?}\nIP Prefix: {:?}\nNetwork Count: {:?}",
            self.cpu_vendor.as_ref().unwrap_or(&String::from("N/A")),
            self.ip_address,
            self.ip_prefix,
            self.network_count,
        )
    }
}
