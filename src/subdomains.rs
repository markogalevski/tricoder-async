use std::{collections::HashSet, time::Duration};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

use crate::{
    error::Error,
    model::{CrtShEntry, Subdomain},
};
use rayon::prelude::*;
use reqwest::blocking::Client;
pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_owned())
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|subdomain| subdomain != target)
        .filter(|subdomain| !subdomain.contains('*'))
        .collect();
    subdomains.insert(target.to_owned());
    Ok(subdomains
        .into_par_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: vec![],
        })
        .filter(resolves)
        .collect())
}

fn resolves(domain: &Subdomain) -> bool {
    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.timeout = Duration::from_secs(4);
    let dns_resolver = Resolver::new(ResolverConfig::default(), resolver_opts)
        .expect("Buildilng dns client shouldn't fail");
    dns_resolver.lookup_ip(&domain.domain).is_ok()
}
