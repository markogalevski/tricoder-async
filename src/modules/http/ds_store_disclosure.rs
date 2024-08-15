use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;

pub struct DsStoreDisclosure {}

#[async_trait]
impl HttpModule for DsStoreDisclosure {
    async fn scan(
        &self,
        http_client: &reqwest::Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, crate::error::Error> {
        let url = format!("{endpoint}/.DS_Store");
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }
        let body = res.bytes().await?;

        if self.is_ds_store_file(&body) {
            return Ok(Some(HttpFinding::DsStoreDisclosure(url)));
        }
        Ok(None)
    }
}

impl Module for DsStoreDisclosure {
    fn name(&self) -> String {
        "http/DsStoreDisclosure".to_owned()
    }

    fn description(&self) -> String {
        "Checks if .DS_Store (Mac) files have been disclosed".to_owned()
    }
}

impl DsStoreDisclosure {
    pub fn new() -> Self {
        Self {}
    }

    fn is_ds_store_file(&self, content: &[u8]) -> bool {
        if content.len() < 8 {
            return false;
        }
        let signature = [0x0, 0x0, 0x0, 0x1, 0x42, 0x75, 0x64, 0x31];
        content[0..8] == signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ds_store() {
        let module = DsStoreDisclosure::new();
        let not_ds_store_body = "testtesttest";
        let ds_store_body = [
            0x0, 0x0, 0x0, 0x1, 0x42, 0x75, 0x64, 0x31, 0xDE, 0xAD, 0xBE, 0xEF,
        ];
        assert!(!module.is_ds_store_file(not_ds_store_body.as_bytes()));
        assert!(module.is_ds_store_file(&ds_store_body))
    }
}
