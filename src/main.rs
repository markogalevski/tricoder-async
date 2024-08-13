mod common_ports;
mod error;
mod model;
mod ports;
mod subdomains;

use anyhow::anyhow;
use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::time::Duration;
use subdomains::enumerate;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!(error::Error::CliUsage));
    }
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(Duration::from_secs(5))
        .build()?;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build()?;
    pool.install(|| {
        let scan_result: Vec<model::Subdomain> = enumerate(&http_client, &args[1])
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();
        for subdomain in scan_result {
            println!("{}:", subdomain.domain);
            for port in subdomain.open_ports {
                println!("\t{}", port.port);
            }
        }
    });
    Ok(())
}
