use super::response_scraper::ResponseScraper;
use crate::discoverer::http::{Response, Url};

pub struct SubPathsScraper {
    response_scraper: ResponseScraper,
}

impl<'a> SubPathsScraper {
    pub fn new() -> Self {
        return Self {
            response_scraper: ResponseScraper::new(),
        };
    }

    pub fn scrap(
        &self,
        base_url: &'a Url,
        response: &Response,
    ) -> impl Iterator<Item = Url> + 'a {
        let urls = self.response_scraper.scrap(response);
        return SubPathsScraperIterator::new(base_url, Box::new(urls));
    }

    
}

fn is_a_base_url_subpath(base_url: &Url, url: &Url) -> bool {
        return url.as_str().starts_with(base_url.as_str());
}

pub struct SubPathsScraperIterator<'a> {
    base_url: &'a Url,
    urls: Box<dyn Iterator<Item = Url>>,
}

impl<'a> SubPathsScraperIterator<'a> {
    pub fn new(base_url: &'a Url, urls: Box<dyn Iterator<Item = Url>>) -> Self {
        return Self { base_url, urls };
    }
}

impl<'a> Iterator for SubPathsScraperIterator<'a> {
    type Item = Url;

    fn next(&mut self) -> Option<Url> {
        loop {
            let url = self.urls.next()?;
            if is_a_base_url_subpath(self.base_url, &url) {
                return Some(url);
            }
        }
    }
}
