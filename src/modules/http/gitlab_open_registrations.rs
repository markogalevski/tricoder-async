use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;
pub struct GitlabOpenRegistrations {}

impl GitlabOpenRegistrations {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module for GitlabOpenRegistrations {
    fn name(&self) -> String {
        String::from("http/gitlab_open_registration")
    }

    fn description(&self) -> String {
        String::from("Check if the GitLab instance is open to registrations")
    }
}
#[async_trait]
impl HttpModule for GitlabOpenRegistrations {
    async fn scan(
        &self,
        http_client: &reqwest::Client,
        endpoint: &str,
    ) -> Result<Option<crate::modules::HttpFinding>, crate::error::Error> {
        let res = http_client.get(endpoint).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }
        let body = res.text().await?;
        if body.contains("This is a self-managed instance of GitLab") && body.contains("Register") {
            return Ok(Some(HttpFinding::GitlabOpenRegistrations(
                endpoint.to_owned(),
            )));
        }
        Ok(None)
    }
}
