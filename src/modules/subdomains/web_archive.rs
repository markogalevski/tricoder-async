use async_trait::async_trait;
use std::collections::HashSet;

use crate::{
    error::Error,
    modules::{Module, SubdomainModule},
};
use url::Url;

pub struct WebArchive {}

impl Module for WebArchive {
    fn name(&self) -> String {
        "subdomain/web_archive".to_owned()
    }

    fn description(&self) -> String {
        "Use web.archive.org to search for subdomains".to_owned()
    }
}

#[async_trait]
impl SubdomainModule for WebArchive {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error> {
        let url = format!("https://web.archive.org/cdx/search/cdx?matchType=domain&fl=original&output=json&collapse=urlkey&url={domain}");
        let res = reqwest::get(&url).await?;
        if !res.status().is_success() {
            return Err(Error::InvalidHttpResponse(self.name()));
        }
        let web_archive_urls: Vec<Vec<String>> = res
            .json()
            .await
            .map_err(|_| Error::InvalidHttpResponse(self.name()))?;
        let subdomains: HashSet<String> = web_archive_urls
            .into_iter()
            .flatten()
            .filter_map(|url| {
                if url == "original" {
                    return None;
                }
                Url::parse(&url)
                    .map_err(|e| {
                        log::error!("{}: error parsing url {url}: {e}", self.name());
                        e
                    })
                    .ok()
            })
            .filter_map(|url| url.host_str().map(|host| host.to_owned()))
            .collect();
        Ok(subdomains.into_iter().collect())
    }
}

impl WebArchive {
    pub fn new() -> Self {
        Self {}
    }
}
