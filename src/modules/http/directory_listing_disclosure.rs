use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

use crate::{
    error::Error,
    modules::{HttpFinding, HttpModule, Module},
};
pub struct DirectoryListingDisclosure {
    dir_listing_regex: Regex,
}

impl Module for DirectoryListingDisclosure {
    fn name(&self) -> String {
        "http/dir_listing_disclosure".to_owned()
    }

    fn description(&self) -> String {
        "Check for enabled directory listing, which often leaks information".to_owned()
    }
}

#[async_trait]
impl HttpModule for DirectoryListingDisclosure {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{endpoint}/");
        let res = http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }
        let body = res.text().await?;
        if self.is_directory_listing(body).await? {
            return Ok(Some(HttpFinding::DirectoryListingDisclosure(url)));
        }
        Ok(None)
    }
}

impl DirectoryListingDisclosure {
    pub fn new() -> Self {
        Self {
            dir_listing_regex: Regex::new(r"<title>Index of .*</title>")
                .expect("Compiling http/DirectoryListingDisclosure"),
        }
    }

    async fn is_directory_listing(&self, body: String) -> Result<bool, Error> {
        let dir_listing_regex = self.dir_listing_regex.clone();
        let res = tokio::task::spawn_blocking(move || dir_listing_regex.is_match(&body)).await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn is_directory_listing() {
        let module = DirectoryListingDisclosure::new();
        let body = "Content <title>Index of galev.ski</title> test".to_owned();
        assert!(module.is_directory_listing(body).await.unwrap());
        let body = "".to_owned();
        assert!(!module.is_directory_listing(body).await.unwrap());
        let body = "Cont cc<ticcctle>Indaaaex of galev.ski</tifle> test".to_owned();
        assert!(!module.is_directory_listing(body).await.unwrap());
        let body = "Content <ttttt>Index of galev.ski</title> test".to_owned();
        assert!(!module.is_directory_listing(body).await.unwrap());
        let body = "Content <title>Index of galev.ski</asdff> test".to_owned();
        assert!(!module.is_directory_listing(body).await.unwrap());
    }
}
