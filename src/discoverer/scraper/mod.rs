mod html_scraper;
mod javascript_scraper;
mod response_scraper;
mod scraper_provider;
mod subpaths_scraper;
mod url_combinator;

pub use scraper_provider::{
    EmptyScraperProvider, ScraperProvider, UrlsScraperProvider,
};
