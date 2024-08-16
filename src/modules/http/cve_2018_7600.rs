use crate::{
    error::Error,
    modules::{HttpFinding, HttpModule, Module},
};
use async_trait::async_trait;
use reqwest::Client;

pub struct Cve2018_7600 {}

impl Module for Cve2018_7600 {
    fn name(&self) -> String {
        "http/cve_2018_7600".to_owned()
    }

    fn description(&self) -> String {
        "Check for CVE-2018-7600 (see: https://nvd.nist.gov/vuln/detail/CVE-2018-7600)".to_owned()
    }
}

#[async_trait]
impl HttpModule for Cve2018_7600 {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{endpoint}/plugins/servlet/oauth/users/icon-uri?consumerUri=https://google.com/robots.txt");
        let res = http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }
        let body = res.text().await?;
        if body.contains("user-agent: *") && body.contains("disallow") {
            return Ok(Some(HttpFinding::Cve2018_7600(url)));
        }
        Ok(None)
    }
}

impl Cve2018_7600 {
    pub fn new() -> Self {
        Self {}
    }
}
