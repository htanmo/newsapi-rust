#[derive(Debug, Default, Clone)]
pub struct TopHeadlinesParams {
    pub country: Option<String>,
    pub category: Option<String>,
    pub sources: Option<String>,
    pub q: Option<String>,
    pub page_size: Option<u32>,
    pub page: Option<u32>,
}

impl TopHeadlinesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_string());
        self
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    pub fn sources(mut self, sources: &str) -> Self {
        self.sources = Some(sources.to_string());
        self
    }

    pub fn q(mut self, query: &str) -> Self {
        self.q = Some(query.to_string());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}
