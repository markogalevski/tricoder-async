use async_trait::async_trait;
use std::collections::HashSet;

use serde::Deserialize;

use crate::modules::{Module, SubdomainModule};

#[derive(Debug, Clone, Deserialize)]
pub struct CrtShEntry {
    pub name_value: String,
}

pub struct CrtSh {}

impl Module for CrtSh {
    fn name(&self) -> String {
        "subdomain/crtsh".to_owned()
    }

    fn description(&self) -> String {
        "Use crt.sh to search for subdomains".to_owned()
    }
}
#[async_trait]
impl SubdomainModule for CrtSh {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, crate::error::Error> {
        let url = format!("https://crt.sh/?q=%25.{domain}&output=json");

        let res = reqwest::get(&url).await?;
        if !res.status().is_success() {
            return Err(crate::error::Error::InvalidHttpResponse(self.name()));
        }
        let entries: Vec<CrtShEntry> = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(crate::error::Error::InvalidHttpResponse(self.name())),
        };

        let subdomains: HashSet<String> = entries
            .into_iter()
            .map(|entry| {
                entry
                    .name_value
                    .split('\n')
                    .map(|subdomain| subdomain.trim().to_owned())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .filter(|subdomain| !subdomain.contains('*'))
            .collect();
        Ok(subdomains.into_iter().collect())
    }
}

impl CrtSh {
    pub fn new() -> Self {
        Self {}
    }
}
