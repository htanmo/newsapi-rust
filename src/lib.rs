mod errors;
mod models;
mod params;

use reqwest::Client as HttpClient;

pub use crate::{
    errors::{NewsApiError, Result},
    models::NewsResponse,
    params::TopHeadlinesParams,
};

#[derive(Debug, Clone, Copy)]
pub enum Version {
    V2,
}

impl Version {
    fn path(&self) -> &'static str {
        match self {
            Version::V2 => "/v2",
        }
    }
}

#[derive(Debug)]
pub struct NewsApiClient {
    api_key: String,
    base_url: String,
    http: HttpClient,
}

impl NewsApiClient {
    const BASE_URL: &str = "https://newsapi.org";

    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_version(api_key, Version::V2)
    }

    pub fn with_version(api_key: impl Into<String>, version: Version) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: format!("{}{}", Self::BASE_URL, version.path()),
            http: HttpClient::new(),
        }
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<NewsResponse> {
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body: serde_json::Value = response.json().await?;
            let message = error_body["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();

            Err(NewsApiError::Api {
                status: status.as_u16(),
                message,
            })
        }
    }

    pub async fn top_headlines(&self, params: &TopHeadlinesParams) -> Result<NewsResponse> {
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

        let url = format!("{}/top-headlines", self.base_url);
        let response = self.http.get(&url).query(&query).send().await?;

        self.handle_response(response).await
    }
}
