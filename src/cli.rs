use std::collections::HashSet;
use std::time::{Duration, Instant};

use futures::stream;
use futures::StreamExt;
use reqwest::Client;

use crate::modules::HttpModule;
use crate::{
    dns,
    modules::{all_http_modules, all_subdomains_modules, Subdomain},
    ports,
};

pub fn modules() -> Result<(), anyhow::Error> {
    let http_modules = all_http_modules();
    let subdomain_modules = all_subdomains_modules();
    println!("\nSubdomain Modules:");
    for module in subdomain_modules {
        println!("\t\t{}: {}", module.name(), module.description())
    }
    println!("\nHTTP Modules:");
    for module in http_modules {
        println!("\t\t{}: {}", module.name(), module.description())
    }

    Ok(())
}

pub fn scan(target: &str) -> Result<(), anyhow::Error> {
    log::info!("Scanning {target}");
    /* Setup stage */
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    let dns_resolver = dns::new_resolver();

    let subdomains_concurrency = 20;
    let dns_concurrency = 100;
    let ports_concurrency = 200;
    let vulnerabilities_concurrency = 20;
    let start = Instant::now();

    let subdomains_modules = all_subdomains_modules();

    runtime.block_on(async move {
        // Enumerate all subdomains using all enumeration methods
        let mut subdomains: Vec<String> = stream::iter(subdomains_modules)
            .map(|module| async move {
                module
                    .enumerate(target)
                    .await
                    .inspect_err(|e| log::error!("subdomains/{}: {}", module.name(), e))
                    .ok()
            })
            .buffer_unordered(subdomains_concurrency)
            .filter_map(|domain| async { domain })
            .collect::<Vec<Vec<String>>>()
            .await
            .into_iter()
            .flatten()
            .collect();
        subdomains.push(target.to_owned());

        // Clean and dedup subdomains
        let subdomains: Vec<Subdomain> = HashSet::<String>::from_iter(subdomains)
            .into_iter()
            .filter_map(|subdomain| {
                if subdomain.contains(target) {
                    Some(Subdomain {
                        domain: subdomain,
                        open_ports: vec![],
                    })
                } else {
                    None
                }
            })
            .collect();
        log::info!("Found {} subdomains", subdomains.len());

        let subdomains: Vec<Subdomain> = stream::iter(subdomains)
            .map(|domain| dns::resolves(&dns_resolver, domain))
            .buffer_unordered(dns_concurrency)
            .filter_map(|domain| async move { domain })
            .collect()
            .await;
        let subdomains: Vec<Subdomain> = stream::iter(subdomains)
            .map(|domain| {
                log::info!("Scanning ports for {}", domain.domain);
                ports::scan_ports(ports_concurrency, domain)
            })
            .buffer_unordered(1)
            .collect()
            .await;
        for subdomain in &subdomains {
            println!("{}", subdomain.domain);
            for port in &subdomain.open_ports {
                println!("\t\t{port}");
            }
        }
        println!("<-------------------- Vulnerabilities -------------------->");

        let mut targets: Vec<(Box<dyn HttpModule>, String)> = vec![];
        for subdomain in subdomains {
            for port in subdomain.open_ports {
                let http_modules = all_http_modules();
                for http_module in http_modules {
                    let target = format!("http://{}/{}", subdomain.domain, port.port);
                    targets.push((http_module, target));
                }
            }
        }
        stream::iter(targets.into_iter())
            .for_each_concurrent(vulnerabilities_concurrency, |(module, target)| {
                let http_client = http_client.clone();
                async move {
                    if let Some(finding) = module
                        .scan(&http_client, &target)
                        .await
                        .inspect_err(|e| log::debug!("Error: {e}"))
                        .ok()
                        .flatten()
                    {
                        println!("{finding:?}");
                    }
                }
            })
            .await;
    });
    log::info!("Scan took {:?}", start.elapsed());

    Ok(())
}
