use std::time::{Duration, Instant};

use futures::stream;
use futures::stream::StreamExt;
use reqwest::Client;

use crate::{model, ports, subdomains::enumerate};

pub fn modules() -> Result<(), anyhow::Error> {
    unimplemented!()
}

pub async fn scan(target: String) -> Result<(), anyhow::Error> {
    let http_client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let ports_concurrency = 200;
    let sd_concurrency = 100;
    let scan_start = Instant::now();
    let subdomains = enumerate(&http_client, &target).await?;
    let scan_result: Vec<model::Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_ports(ports_concurrency, subdomain))
        .buffer_unordered(sd_concurrency)
        .collect()
        .await;

    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {scan_duration:?}");
    for subdomain in scan_result {
        println!("{}:", subdomain.domain);
        for port in subdomain.open_ports {
            println!("\t{}", port.port);
        }
    }
    Ok(())
}
