mod html_scraper;
mod javascript_scraper;
mod scraper_manager;
mod response_scraper;

pub use scraper_manager::{
    EmptyScraperManager, HtmlScraperManager, ScraperManager,
};
