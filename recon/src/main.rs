mod host_system;

use std::net::IpAddr;

//src/main.rs
use sysinfo::{NetworkData, Networks, System};
use serde::Serialize;
use core::net::Ipv4Addr;
use serde_json::json;
use crate::host_system::HostSystemBuilder;


async fn fetch_cpu_vendors(sys: &System) -> Vec<String> {
    // let mut structs: Vec<SystemInfo> = Vec::new();
    let mut cpu_vendors: Vec<String> = Vec::new();
    for cpu in sys.cpus() {
        let brand = cpu.brand().to_string();
        if !(cpu_vendors.contains(&brand)) {
            cpu_vendors.push(brand)
        }
    }
return cpu_vendors;
}


#[tokio::main]
async fn main() {

    let mut sys = System::new_all();
    let networks = sysinfo::Networks::new_with_refreshed_list();
    let mut ip_addresses: Vec<IpAddr> = Vec::new();

    for (interface_name, network )in networks.iter() {
        println!("Ip Networks: {:?}", network.ip_networks());
        for ip_network in network.ip_networks() {
            ip_addresses.push(ip_network.addr);
        }
    }

    sys.refresh_all();

    let mut local_host_system = HostSystemBuilder::new();


    let cpu_vendors = fetch_cpu_vendors(&sys).await;
    local_host_system.cpu_vendor(cpu_vendors[0].as_str());
    local_host_system.ip_prefix(127);
    local_host_system.ip_address(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    local_host_system.network_count(1);
    let the_system = local_host_system.build();
    print!("{}", the_system);

    

    
}