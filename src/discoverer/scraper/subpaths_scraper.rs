use super::response_scraper::ResponseScraper;
use crate::discoverer::http::{Response, Url};

pub struct SubPathsScraper {
    response_scraper: ResponseScraper,
}

impl SubPathsScraper {
    pub fn new() -> Self {
        return Self {
            response_scraper: ResponseScraper::new(),
        };
    }

    pub fn scrap(
        &self,
        base_url: &'static Url,
        response: &Response,
    ) -> impl Iterator<Item = Url> {
        let urls = self.response_scraper.scrap(response);
        return urls.filter(|u| Self::is_a_base_url_subpath(base_url, u));
    }

    fn is_a_base_url_subpath(base_url: &Url, url: &Url) -> bool {
        return url.as_str().starts_with(base_url.as_str());
    }
}
