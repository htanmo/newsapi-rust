use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Rate Limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid Parameters: {0}")]
    InvalidParams(String),
}

pub type Result<T> = std::result::Result<T, NewsApiError>;
