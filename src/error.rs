use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignerError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("json data sent from server is corrupted: ")]
    DataCorrupted(#[from] json::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("endpoint error: {0}")]
    EndpointError(String),
    #[error("invalid input")]
    InvalidInput,
    #[error("config error: {0}")]
    ConfigError(&'static str)
}
