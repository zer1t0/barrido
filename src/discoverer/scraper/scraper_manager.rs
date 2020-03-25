use crate::discoverer::http::{Response, ContentType};
use super::html_scraper::HtmlScraper;
use super::javascript_scraper::JsScraper;
use reqwest::Url;
use crate::discoverer::communication::{UrlsMessage, UrlsSender};

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
    html_scraper: HtmlScraper,
    js_scraper: JsScraper,
    discovered_paths_sender: UrlsSender,
}

impl HtmlScraperManager {
    pub fn new(discovered_paths_sender: UrlsSender) -> Self {
        return Self {
            html_scraper: HtmlScraper::new(),
            js_scraper: JsScraper::new(),
            discovered_paths_sender,
        };
    }

    fn scrap_html(&self, base_url: Url, response: &Response) {
        let mut urls = self.html_scraper.scrap(response);
        urls.retain(|url| self.is_a_base_url_subpath(&base_url, &url));

        let urls_message = UrlsMessage::new(base_url, urls);

        self.send_paths(urls_message);
    }

    fn scrap_javascript(&self, base_url: Url, response: &Response) {
        let mut urls = self.js_scraper.scrap(response);
        urls.retain(|url| self.is_a_base_url_subpath(&base_url, &url));

        let urls_message = UrlsMessage::new(base_url, urls);
        self.send_paths(urls_message);
    }

    fn send_paths(&self, message: UrlsMessage) {
        info!("send {} paths", message.urls.len());
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
        if let Some(content_type) = response.content_type() {
            match content_type {
                ContentType::Html => self.scrap_html(base_url, response),
                ContentType::Javascript => {
                    self.scrap_javascript(base_url, response)
                }
            }
        }
    }
}
