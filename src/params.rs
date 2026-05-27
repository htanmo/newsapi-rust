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

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn sources(mut self, sources: impl Into<String>) -> Self {
        self.sources = Some(sources.into());
        self
    }

    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page.max(1));
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size.min(100));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct EverythingParams {
    pub q: Option<String>,
    pub q_in_title: Option<String>,
    pub sources: Option<String>,
    pub domains: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub language: Option<String>,
    pub sort_by: Option<String>,
    pub page_size: Option<u32>,
    pub page: Option<u32>,
}

impl EverythingParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn q(mut self, query: impl Into<String>) -> Self {
        self.q = Some(query.into());
        self
    }

    pub fn q_in_title(mut self, query: impl Into<String>) -> Self {
        self.q_in_title = Some(query.into());
        self
    }

    pub fn sources(mut self, sources: impl Into<String>) -> Self {
        self.sources = Some(sources.into());
        self
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn domains(mut self, domains: impl Into<String>) -> Self {
        self.domains = Some(domains.into());
        self
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn sort_by(mut self, sort: impl Into<String>) -> Self {
        self.sort_by = Some(sort.into());
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size.min(100));
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page.max(1));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct SourceParams {
    pub category: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
}

impl SourceParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }
}
