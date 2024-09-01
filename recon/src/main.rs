//src/main.rs
mod host_system;
use std::net::IpAddr;
use sysinfo::{NetworkData, Networks, System};
use serde::Serialize;
use core::net::Ipv4Addr;
use serde_json::json;
use crate::host_system::HostSystemBuilder;
use reqwest::{Client, Error};


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


async fn post_hello() -> Result<(), Error> {
    let url = "http://127.0.0.1:5000/client-hello";
    let client = reqwest::Client::new();
    let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    let response = client
    .post(url)
    .header("Content-Type", "application/json")
    .body(json_data.to_owned())
    .send()
    .await?;

    println!("Status: {}", response.status());
    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error> {

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
    println!("{}", the_system);
    post_hello().await?;
    Ok(())

    

    
}