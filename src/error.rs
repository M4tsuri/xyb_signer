use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignerError {
    #[error("network error: ")]
    NetworkError(#[from] reqwest::Error),
    #[error("json data sent from server is corrupted: ")]
    DataCorrupted(#[from] json::Error),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("verify code error: {0}")]
    VerifyCodeError(String),
    #[error("login failed: {0}")]
    LoginFailed(String),
    #[error("endpoint error: {0}")]
    EndpointError(String),
    #[error("invalid input")]
    InvalidInput,
    #[error("config error: {0}")]
    ConfigError(&'static str)
}
