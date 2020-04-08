use super::subpaths_scraper::SubPathsScraper;
use crate::discoverer::communication::{UrlsMessage, UrlsSender};
use crate::discoverer::http::{Response, Url};

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
    subpaths_scraper: SubPathsScraper,
    discovered_paths_sender: UrlsSender,
}

impl HtmlScraperManager {
    pub fn new(discovered_paths_sender: UrlsSender) -> Self {
        return Self {
            subpaths_scraper: SubPathsScraper::new(),
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
}

impl ScraperManager for HtmlScraperManager {
    fn scrap_response(&self, base_url: Url, response: &Response) {
        let urls = self.subpaths_scraper.scrap(&base_url, response).collect();
        let urls_message = UrlsMessage::new(base_url, urls);
        self.send_urls(urls_message);
    }
}
