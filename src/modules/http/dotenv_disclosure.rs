use crate::{
    error::Error,
    modules::{HttpFinding, HttpModule, Module},
};
use async_trait::async_trait;

pub struct DotenvDisclosure {}

#[async_trait]
impl HttpModule for DotenvDisclosure {
    async fn scan(
        &self,
        http_client: &reqwest::Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{endpoint}/.env");
        let res = http_client.get(&url).send().await?;
        if res.status().is_success() {
            return Ok(Some(HttpFinding::DotenvDisclosure(url)));
        }
        Ok(None)
    }
}

impl Module for DotenvDisclosure {
    fn name(&self) -> String {
        "http/.env_disclosure".to_owned()
    }

    fn description(&self) -> String {
        "Check if .env files have been disclosed".to_owned()
    }
}

impl DotenvDisclosure {
    pub fn new() -> Self {
        Self {}
    }
}
