use std::sync::Arc;
use std::time::Duration;

use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::name_server::{GenericConnector, TokioRuntimeProvider};
use hickory_resolver::AsyncResolver;

use crate::modules::Subdomain;

type DnsResolver = Arc<AsyncResolver<GenericConnector<TokioRuntimeProvider>>>;

pub async fn resolves(resolver: &DnsResolver, domain: Subdomain) -> Option<Subdomain> {
    if resolver.lookup_ip(&domain.domain).await.is_ok() {
        Some(domain)
    } else {
        None
    }
}

pub fn new_resolver() -> DnsResolver {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);
    Arc::new(AsyncResolver::tokio(ResolverConfig::default(), opts))
}
