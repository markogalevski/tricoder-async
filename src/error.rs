use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Usage: tricoder <website.com>")]
    CliUsage,
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}
