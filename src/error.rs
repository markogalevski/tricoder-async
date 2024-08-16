use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Tokio: {0}")]
    TokioJoin(#[from] tokio::task::JoinError),
    #[error("Invalid Http Response, module {0}")]
    InvalidHttpResponse(String),
}
