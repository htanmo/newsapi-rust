//! The main client for interacting with NewsAPI.

use std::time::Duration;

use reqwest::Client as HttpClient;

pub use crate::{
    error::{NewsApiError, Result},
    models::{ApiResponse, SourceDetail, SourcesResponse, SuccessResponse},
    params::{EverythingParams, SourceParams, TopHeadlinesParams},
    version::ApiVersion,
};

const BASE_URL: &str = "https://newsapi.org";

/// The main NewsAPI client.
///
/// Holds your API key, HTTP client configuration, and provides methods for all API endpoints.
/// All methods are asynchronous and must be `await`ed.
///
/// # Examples
/// ```no_run
/// use newsapi_rust::{NewsApiClient, TopHeadlinesParams};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = NewsApiClient::new("YOUR_API_KEY");
///     let params = TopHeadlinesParams::new().country("us").category("technology");
///     let headlines = client.top_headlines(&params).await?;
///     for article in headlines.articles {
///         println!("{} - {}", article.title, article.url);
///     }
///     Ok(())
/// }
#[derive(Debug, Clone)]
pub struct NewsApiClient {
    api_key: String,
    timeout: Duration,
    http: HttpClient,
    version: ApiVersion,
}

impl NewsApiClient {
    /// Create a new NewsApiClient with the given API key.
    ///
    /// Uses default settings for timeout (30 seconds) and API version (v2).
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(api_key, Duration::from_secs(30), ApiVersion::default())
    }

    /// Create a new NewsApiClient with custom configuration.
    pub fn with_config(api_key: impl Into<String>, timeout: Duration, version: ApiVersion) -> Self {
        let http = HttpClient::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http,
            api_key: api_key.into(),
            version,
            timeout,
        }
    }

    /// Create a new NewsApiClient with a custom timeout.
    pub fn with_timeout(api_key: impl Into<String>, timeout: Duration) -> Self {
        Self::with_config(api_key, timeout, ApiVersion::V2)
    }

    /// Create a new NewsApiClient with a specific API version.
    pub fn with_version(api_key: impl Into<String>, version: ApiVersion) -> Self {
        Self::with_config(api_key, Duration::from_secs(30), version)
    }

    /// Set the API version for this client.
    pub fn set_version(&mut self, version: ApiVersion) {
        self.version = version;
    }

    /// Get the current API version.
    pub fn version(&self) -> ApiVersion {
        self.version
    }

    /// Set the timeout for HTTP requests. Rebuilds the internal HTTP client with the new timeout.
    pub fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
        let new_http = HttpClient::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| NewsApiError::Request(e.into()))?;
        self.http = new_http;
        self.timeout = timeout;
        Ok(())
    }

    // Helper method to build the full URL for a given endpoint
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}/{}", BASE_URL, self.version.path(), endpoint)
    }

    // Helper method to execute a request with error handling
    async fn execute_request(
        &self,
        request: reqwest::RequestBuilder,
        timeout: Option<Duration>,
    ) -> Result<reqwest::Response> {
        let request = if let Some(t) = timeout {
            request.timeout(t)
        } else {
            request
        };
        request.send().await.map_err(|e| {
            if e.is_timeout() {
                let actual_timeout = timeout.unwrap_or(self.timeout);
                NewsApiError::Timeout(actual_timeout)
            } else {
                NewsApiError::Request(e)
            }
        })
    }

    // Helper method to handle API responses and convert them into SuccessResponse or NewsApiError
    async fn handle_response(&self, response: reqwest::Response) -> Result<SuccessResponse> {
        let body = response.bytes().await?;
        let api_response: ApiResponse = serde_json::from_slice(&body)?;
        match api_response {
            ApiResponse::Success(success) => Ok(success),
            ApiResponse::Error(error) => {
                if error.code == "rateLimited" {
                    Err(NewsApiError::RateLimitExceeded)
                } else if error.code == "apiKeyInvalid" {
                    Err(NewsApiError::InvalidApiKey)
                } else {
                    Err(NewsApiError::Api {
                        code: error.code,
                        message: error.message,
                    })
                }
            }
            ApiResponse::Unknown => Err(NewsApiError::InvalidParams(
                "Unexpected response format".into(),
            )),
        }
    }

    /// Fetch top headlines using `/top-headlines` endpoint with the given parameters.
    ///
    /// # Arguments
    /// * `params` - A reference to `TopHeadlinesParams` containing the query parameters.
    ///
    /// # Returns
    /// A `Result` containing `SuccessResponse` on success or `NewsApiError` on failure.
    pub async fn top_headlines(&self, params: &TopHeadlinesParams) -> Result<SuccessResponse> {
        self.top_headlines_with_timeout(params, None).await
    }

    /// Fetch top headlines using `/top-headlines` endpoint with the given parameters and an optional timeout.
    ///
    /// # Arguments
    /// * `params` - A reference to `TopHeadlinesParams` containing the query parameters.
    /// * `timeout` - An optional `Duration` specifying the timeout for the request.
    ///
    /// # Returns
    /// A `Result` containing `SuccessResponse` on success or `NewsApiError` on failure.
    pub async fn top_headlines_with_timeout(
        &self,
        params: &TopHeadlinesParams,
        timeout: Option<Duration>,
    ) -> Result<SuccessResponse> {
        let mut query = vec![("apiKey", self.api_key.to_string())];

        if let Some(country) = &params.country {
            query.push(("country", country.to_string()));
        }

        if let Some(category) = &params.category {
            query.push(("category", category.to_string()));
        }

        if let Some(sources) = &params.sources {
            query.push(("sources", sources.to_string()));
        }

        if let Some(q) = &params.q {
            query.push(("q", q.to_string()));
        }

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }

        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        let url = self.build_url("/top-headlines");
        let request = self.http.get(&url).query(&query);
        let response = self.execute_request(request, timeout).await?;
        self.handle_response(response).await
    }

    /// Fetch all articles using the `/everything` endpoint with the given parameters.
    ///
    /// # Arguments
    /// * `params` - A reference to `EverythingParams` containing the query parameters.
    ///
    /// # Returns
    /// A `Result` containing `SuccessResponse` on success or `NewsApiError` on failure.
    pub async fn everything(&self, params: &EverythingParams) -> Result<SuccessResponse> {
        self.everything_with_timeout(params, None).await
    }

    /// Fetch all articles using the `/everything` endpoint with the given parameters and an optional timeout.
    ///
    /// # Arguments
    /// * `params` - A reference to `EverythingParams` containing the query parameters.
    /// * `timeout` - An optional `Duration` specifying the timeout for the request.
    ///
    /// # Returns
    /// A `Result` containing `SuccessResponse` on success or `NewsApiError` on failure.
    pub async fn everything_with_timeout(
        &self,
        params: &EverythingParams,
        timeout: Option<Duration>,
    ) -> Result<SuccessResponse> {
        let mut query = vec![("apiKey", self.api_key.to_string())];
        if let Some(q) = &params.q {
            query.push(("q", q.to_string()));
        }

        if let Some(q_in_title) = &params.q_in_title {
            query.push(("qInTitle", q_in_title.to_string()));
        }

        if let Some(sources) = &params.sources {
            query.push(("sources", sources.to_string()));
        }

        if let Some(domains) = &params.domains {
            query.push(("domains", domains.to_string()));
        }

        if let Some(from) = &params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = &params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(language) = &params.language {
            query.push(("language", language.to_string()));
        }

        if let Some(sort_by) = &params.sort_by {
            query.push(("sortBy", sort_by.to_string()));
        }

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }

        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        let url = self.build_url("/everything");
        let request = self.http.get(&url).query(&query);
        let response = self.execute_request(request, timeout).await?;
        self.handle_response(response).await
    }

    /// Fetch news sources using the `/sources` endpoint with the given parameters.
    ///
    /// # Arguments
    /// * `params` - A reference to `SourceParams` containing the query parameters.
    ///
    /// # Returns
    /// A `Result` containing a vector of `SourceDetail` on success or `NewsApiError` on failure.
    pub async fn sources(&self, params: &SourceParams) -> Result<Vec<SourceDetail>> {
        self.sources_with_timeout(params, None).await
    }

    /// Fetch news sources using the `/sources` endpoint with the given parameters and an optional timeout.
    ///
    /// # Arguments
    /// * `params` - A reference to `SourceParams` containing the query parameters.
    /// * `timeout` - An optional `Duration` specifying the timeout for the request.
    ///
    /// # Returns
    /// A `Result` containing a vector of `SourceDetail` on success or `NewsApiError` on failure.
    pub async fn sources_with_timeout(
        &self,
        params: &SourceParams,
        timeout: Option<Duration>,
    ) -> Result<Vec<SourceDetail>> {
        let mut query = vec![("apiKey", self.api_key.to_string())];

        if let Some(category) = &params.category {
            query.push(("category", category.to_string()));
        }

        if let Some(language) = &params.language {
            query.push(("language", language.to_string()));
        }

        if let Some(country) = &params.country {
            query.push(("country", country.to_string()));
        }

        let url = self.build_url("/sources");
        let request = self.http.get(&url).query(&query);
        let response = self.execute_request(request, timeout).await?;
        let body = response.bytes().await?;

        let source_resp: SourcesResponse = serde_json::from_slice(&body)?;
        match source_resp {
            SourcesResponse::Ok { sources } => Ok(sources),
            SourcesResponse::Err(err) => Err(NewsApiError::Api {
                code: err.code,
                message: err.message,
            }),
            SourcesResponse::Unknown => Err(NewsApiError::InvalidParams(
                "Unknown response from sources endpoint".into(),
            )),
        }
    }
}
