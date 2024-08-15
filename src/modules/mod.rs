use crate::error::Error;
use async_trait::async_trait;
use http::{
    DirectoryListingDisclosure, DotenvDisclosure, DsStoreDisclosure, EtcdUnauthenticatedAccess,
    GitHeadDisclosure, GitlabOpenRegistrations, KibanaUnauthenticatedAccess,
};
use reqwest::Client;
use subdomains::{CrtSh, WebArchive};

mod http;
mod subdomains;

pub fn all_http_modules() -> Vec<Box<dyn HttpModule>> {
    vec![
        Box::new(GitlabOpenRegistrations::new()),
        Box::new(DirectoryListingDisclosure::new()),
        Box::new(DotenvDisclosure::new()),
        Box::new(DsStoreDisclosure::new()),
        Box::new(EtcdUnauthenticatedAccess::new()),
        Box::new(GitHeadDisclosure::new()),
        Box::new(KibanaUnauthenticatedAccess::new()),
    ]
}

pub fn all_subdomains_modules() -> Vec<Box<dyn SubdomainModule>> {
    vec![Box::new(CrtSh::new()), Box::new(WebArchive::new())]
}

enum HttpFinding {
    GitlabOpenRegistrations(String),
    GitHeadDisclosure(String),
    DotenvDisclosure(String),
    DsStoreDisclosure(String),
    EtcdUnauthenticatedAccess(String),
    KibanaUnauthenticatedAccess(String),
    DirectoryListingDisclosure(String),
}

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[async_trait]
pub trait SubdomainModule: Module {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error>;
}

#[async_trait]
pub trait HttpModule: Module {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error>;
}
