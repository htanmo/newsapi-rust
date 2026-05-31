//! # newsapi-rust
//!
//! A robust, async Rust client for the [NewsAPI.org](https://newsapi.org/) news aggregator API.
//!
//! This library provides a type-safe, async interface to all NewsAPI v2 endpoints:
//! - `/top-headlines` - get breaking news headlines
//! - `/everything` - search millions of articles
//! - `/sources` - discover available news sources
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! newsapi-rust = "0.1"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! Then:
//! ```no_run
//! use newsapi_rust::{NewsApiClient, ApiVersion};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client =  NewsApiClient::new("YOUR_API_KEY", ApiVersion::V2);
//!     let params = TopHeadlinesParams::new()
//!         .country("us")
//!        .category("technology");
//!     let response = client.top_headlines(&params).await?;
//!     for article in response.articles {
//!        println!("{} - {}", article.title, article.url);
//!     }
//!     Ok(())
//! }
//! ```

// Ensure all public items are documented
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/newsapi-rust/0.1.0")]

mod client;
mod error;
mod models;
mod params;
mod version;

pub use client::NewsApiClient;
pub use error::{NewsApiError, Result};
pub use models::{Article, Source, SuccessResponse};
pub use params::{EverythingParams, SourceParams, TopHeadlinesParams};
pub use version::ApiVersion;
