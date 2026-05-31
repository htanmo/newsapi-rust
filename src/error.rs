//! Error types used throughout the library.

use std::time::Duration;

use thiserror::Error;

/// The main error type for the News API client library.
#[derive(Error, Debug)]
pub enum NewsApiError {
    /// An error occurred while making the HTTP request.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The API returned an error response.
    #[error("API returned error: {code} - {message}")]
    Api {
        /// The error code returned by the API.
        code: String,
        /// The error message returned by the API.
        message: String,
    },

    /// The API key provided is invalid.
    #[error("Invalid API key")]
    InvalidApiKey,

    /// The API rate limit has been exceeded.
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// The requested resource are semantically wrong or missing required parameters.
    #[error("Invalid Parameters: {0}")]
    InvalidParams(String),

    /// The request timed out.
    #[error("Request timed out after {0:?}")]
    Timeout(Duration),

    /// An error occurred while deserializing the JSON response.
    #[error("Json deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// A specialized Result type for the News API client library.
pub type Result<T> = std::result::Result<T, NewsApiError>;
