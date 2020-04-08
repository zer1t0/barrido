use super::scraper_provider::{ScraperProvider, EmptyScraperProvider, UrlsScraperProvider};

pub struct ScraperFactory{}

impl ScraperFactory {

    pub fn create(use_scraper: bool) -> Box<dyn ScraperProvider> {
        match use_scraper {
            true => return Box::new(UrlsScraperProvider::new()),
            false => return Box::new(EmptyScraperProvider::new())
        }
    }
}
