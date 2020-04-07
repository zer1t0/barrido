use crate::discoverer::http::{Response, Url};
use crate::discoverer::communication::{UrlsMessage, UrlsSender};
use super::response_scraper::ResponseScraper;

use log::info;

pub trait ScraperManager {
    fn scrap_response(&self, base_url: Url, response: &Response);
}

pub struct EmptyScraperManager {}

impl EmptyScraperManager {
    pub fn new(_: UrlsSender) -> Self {
        return Self {};
    }
}

impl ScraperManager for EmptyScraperManager {
    fn scrap_response(&self, _: Url, _: &Response) {}
}

pub struct HtmlScraperManager {
    response_scraper: ResponseScraper,
    discovered_paths_sender: UrlsSender,
}

impl HtmlScraperManager {
    pub fn new(discovered_paths_sender: UrlsSender) -> Self {
        return Self {
            response_scraper: ResponseScraper::new(),
            discovered_paths_sender,
        };
    }

    fn send_urls(&self, message: UrlsMessage) {
        info!("send {} urls", message.urls.len());
        if message.urls.len() != 0 {
            self.discovered_paths_sender
                .send(message)
                .expect("Error sending new path");
        }
    }

    fn is_a_base_url_subpath(&self, base_url: &Url, url: &Url) -> bool {
        return url.as_str().starts_with(base_url.as_str());
    }
}

impl ScraperManager for HtmlScraperManager {
    fn scrap_response(&self, base_url: Url, response: &Response) {
        let urls = self.response_scraper.scrap(response);
        let sub_urls = urls.filter(|u| self.is_a_base_url_subpath(&base_url, u)).collect();
        let urls_message = UrlsMessage::new(base_url, sub_urls);
        self.send_urls(urls_message);
    }
}
