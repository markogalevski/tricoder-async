use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;

pub struct EtcdUnauthenticatedAccess {}

impl Module for EtcdUnauthenticatedAccess {
    fn name(&self) -> String {
        "http/etcd_unauthenticated_access".to_owned()
    }

    fn description(&self) -> String {
        "Check for unauthenticated access possibilities on an etcd database".to_owned()
    }
}

#[async_trait]
impl HttpModule for EtcdUnauthenticatedAccess {
    async fn scan(
        &self,
        http_client: &reqwest::Client,
        endpoint: &str,
    ) -> Result<Option<crate::modules::HttpFinding>, crate::error::Error> {
        let url = format!("{endpoint}/version");
        let res = http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }
        let body = res.text().await?;
        if body.contains(r#""etcdserver""#)
            && body.contains(r#""etcdcluster""#)
            && body.chars().count() < 200
        {
            return Ok(Some(HttpFinding::EtcdUnauthenticatedAccess(url)));
        }
        Ok(None)
    }
}

impl EtcdUnauthenticatedAccess {
    pub fn new() -> Self {
        Self {}
    }
}
