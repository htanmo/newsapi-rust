//! Builder structs for request parameters.

/// Parameters for the `/top-headlines` endpoint.
///
/// See [the API documentation](https://newsapi.org/docs/endpoints/top-headlines) for more details.
#[derive(Debug, Default, Clone)]
pub struct TopHeadlinesParams {
    pub(crate) country: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) sources: Option<String>,
    pub(crate) q: Option<String>,
    pub(crate) page_size: Option<u32>,
    pub(crate) page: Option<u32>,
}

impl TopHeadlinesParams {
    /// Creates a new `TopHeadlinesParams` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the country code (e.g. "us" for United States).
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets the category (e.g. "business", "entertainment", "health", "science", "sports", "technology").
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the news sources (e.g. "bbc-news", "cnn").
    pub fn sources(mut self, sources: impl Into<String>) -> Self {
        self.sources = Some(sources.into());
        self
    }

    /// Sets the search query.
    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    /// Sets the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page.max(1));
        self
    }

    /// Sets the page size.
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size.min(100));
        self
    }
}

/// Parameters for the `/everything` endpoint.
///
/// See [the API documentation](https://newsapi.org/docs/endpoints/everything) for more details.
#[derive(Debug, Clone, Default)]
pub struct EverythingParams {
    pub(crate) q: Option<String>,
    pub(crate) q_in_title: Option<String>,
    pub(crate) sources: Option<String>,
    pub(crate) domains: Option<String>,
    pub(crate) from: Option<String>,
    pub(crate) to: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) sort_by: Option<String>,
    pub(crate) page_size: Option<u32>,
    pub(crate) page: Option<u32>,
}

impl EverythingParams {
    /// Creates a new `EverythingParams` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the search query.
    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    /// Sets the search query to only look in article titles.
    pub fn q_in_title(mut self, query: impl Into<String>) -> Self {
        self.q_in_title = Some(query.into());
        self
    }

    /// Sets the news sources.
    pub fn sources(mut self, sources: impl Into<String>) -> Self {
        self.sources = Some(sources.into());
        self
    }

    /// Sets the start date for the search range.
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the end date for the search range.
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Sets the domains for the search range.
    pub fn domains(mut self, domains: impl Into<String>) -> Self {
        self.domains = Some(domains.into());
        self
    }

    /// Sets the language for the search results (e.g. "en" for English).
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Sets the sorting method for the search results (e.g. "relevancy", "popularity", "publishedAt").
    pub fn sort_by(mut self, sort: impl Into<String>) -> Self {
        self.sort_by = Some(sort.into());
        self
    }

    /// Sets the page size for the search results.
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size.min(100));
        self
    }

    /// Sets the page number for the search results.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page.max(1));
        self
    }
}

/// Parameters for the `/sources` endpoint.
///
/// See [the API documentation](https://newsapi.org/docs/endpoints/sources) for more details.
#[derive(Debug, Clone, Default)]
pub struct SourceParams {
    pub(crate) category: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) country: Option<String>,
}

impl SourceParams {
    /// Creates a new `SourceParams` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the category for the news sources.
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the language for the news sources.
    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Sets the country for the news sources.
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
}
