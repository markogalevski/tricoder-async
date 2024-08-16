use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;

pub struct KibanaUnauthenticatedAccess {}

impl Module for KibanaUnauthenticatedAccess {
    fn name(&self) -> String {
        "http/kibana_unauthenticated_access".to_owned()
    }

    fn description(&self) -> String {
        "Check for Kibana databases the can be accessed without Auth".to_owned()
    }
}

#[async_trait]
impl HttpModule for KibanaUnauthenticatedAccess {
    async fn scan(
        &self,
        http_client: &reqwest::Client,
        endpoint: &str,
    ) -> Result<Option<crate::modules::HttpFinding>, crate::error::Error> {
        let url = endpoint.to_owned();
        let res = http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;

        if body.contains(r#"</head><body kbn-chrome id="kibana-body"><kbn-initial-state"#) 
            || body.contains(r#"<div class="ui-app-loading"><h1><strong>Kibana</strong><small>&nbsp;isloading."#)
            || Some(0) == body.find(r#"|| body.contains("#) 
            || body.contains(r#"<div class="kibanaWelcomeLogo"></div></div></div><div class="kibanaWelcomeText">Loading Kibana</div></div>"#) {
                return Ok(Some(HttpFinding::KibanaUnauthenticatedAccess(url)));
            }
        Ok(None)
    }
}

impl KibanaUnauthenticatedAccess {
    pub fn new() -> Self {
        Self {}
    }
}
