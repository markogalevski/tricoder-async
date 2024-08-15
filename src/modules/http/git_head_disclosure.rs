use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;

pub struct GitHeadDisclosure {}
impl GitHeadDisclosure {
    pub fn new() -> Self {
        Self {}
    }
    fn is_head_file(&self, content: String) -> bool {
        content.to_lowercase().trim().find("ref:") == Some(0)
    }
}

#[async_trait]
impl HttpModule for GitHeadDisclosure {
    async fn scan(
        &self,
        http_client: &reqwest::Client,
        endpoint: &str,
    ) -> Result<Option<crate::modules::HttpFinding>, crate::error::Error> {
        let url = format!("{endpoint}/.git/HEAD");
        let res = http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }

        let body: String = res.text().await?;
        if self.is_head_file(body) {
            return Ok(Some(HttpFinding::GitHeadDisclosure(url)));
        }
        Ok(None)
    }
}

impl Module for GitHeadDisclosure {
    fn name(&self) -> String {
        "http/git_head_disclosure".to_owned()
    }

    fn description(&self) -> String {
        "Check for .git/HEAD file disclosure".to_owned()
    }
}
