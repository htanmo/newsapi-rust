//! API Version abstraction.

/// API version enum for NewsAPI. Currently only supports v2, but designed for future extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApiVersion {
    /// Version 2 of the NewsAPI.
    #[default]
    V2,
}

impl ApiVersion {
    /// Get the API path segment for this version. (e.g. `"/v2"`).
    pub fn path(&self) -> &'static str {
        match self {
            ApiVersion::V2 => "/v2",
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiVersion::V2 => write!(f, "v2"),
        }
    }
}
