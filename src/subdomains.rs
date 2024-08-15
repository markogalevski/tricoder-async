use futures::stream;
use futures::stream::StreamExt;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::name_server::{GenericConnector, TokioRuntimeProvider};
use hickory_resolver::AsyncResolver;
use std::{collections::HashSet, time::Duration};

use crate::{
    error::Error,
    model::{CrtShEntry, Subdomain},
};
use reqwest::Client;

pub async fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()
        .await?
        .json()
        .await?;
    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.timeout = Duration::from_secs(4);
    let dns_resolver = AsyncResolver::tokio(ResolverConfig::default(), resolver_opts);

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
    let subdomains = stream::iter(subdomains.into_iter())
        .map(|domain| Subdomain {
            domain,
            open_ports: vec![],
        })
        .filter_map(|subdomain| {
            let dns_resolver = dns_resolver.clone();
            async move {
                if resolves(&dns_resolver, &subdomain).await {
                    Some(subdomain)
                } else {
                    None
                }
            }
        })
        .collect()
        .await;
    Ok(subdomains)
}

pub async fn resolves(
    dns_resolver: &AsyncResolver<GenericConnector<TokioRuntimeProvider>>,
    domain: &Subdomain,
) -> bool {
    let resolved = dns_resolver.lookup_ip(&domain.domain).await;
    resolved.is_ok()
}
