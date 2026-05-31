//! Data structures representing the API responses.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a news article returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// The source of the article, containing its ID and name.
    pub source: Source,
    /// The author of the article, if available.
    pub author: Option<String>,
    /// The title of the article.
    pub title: String,
    /// A brief description of the article, if available.
    pub description: Option<String>,
    /// The main content of the article, if available.
    pub content: Option<String>,
    /// The URL to the full article.
    pub url: String,
    /// The URL to the article's image, if available.
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    /// The publication date and time of the article.
    #[serde(rename = "publishedAt")]
    pub published_at: DateTime<Utc>,
}

/// Represents the source of a news article, containing its ID and name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// The unique identifier of the source, if available.
    pub id: Option<String>,
    /// The name of the source.
    pub name: String,
}

/// Represents the response from the API, which can be either a success or an error.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum ApiResponse {
    /// Indicates a successful response, containing the total number of results and a list of articles.
    #[serde(rename = "ok")]
    Success(SuccessResponse),
    /// Indicates an error response, containing an error code and message.
    #[serde(rename = "error")]
    Error(ErrorResponse),
    /// Represents an unknown response status that is not explicitly handled.
    #[serde(other)]
    Unknown,
}

/// Represents a successful response from the API, containing the total number of results and a list of articles.
#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResponse {
    /// The total number of results available for the query.
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    /// A list of articles returned by the API.
    pub articles: Vec<Article>,
}

/// Represents an error response from the API, containing an error code and message.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// The error code returned by the API, indicating the type of error that occurred.
    pub code: String,
    /// A human-readable message providing more details about the error.
    pub message: String,
}

/// Represents the response from the API when requesting news sources, which can be either a success or an error.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum SourcesResponse {
    /// Indicates a successful response, containing a list of news sources.
    #[serde(rename = "ok")]
    Ok { sources: Vec<SourceDetail> },
    /// Indicates an error response, containing an error code and message.
    #[serde(rename = "error")]
    Err(ErrorResponse),
    #[serde(other)]
    /// Represents an unknown response status that is not explicitly handled.
    Unknown,
}

/// Represents the details of a news source.
#[derive(Debug, Clone, Deserialize)]
pub struct SourceDetail {
    /// The unique identifier of the news source.
    pub id: String,
    /// The name of the news source.
    pub name: String,
    /// A brief description of the news source.
    pub description: String,
    /// The URL of the news source's website.
    pub url: String,
    /// The category of news that the source primarily covers (e.g., "technology", "sports").
    pub category: String,
    /// The language of the news source.
    pub language: String,
    /// The country of the news source.
    pub country: String,
}
