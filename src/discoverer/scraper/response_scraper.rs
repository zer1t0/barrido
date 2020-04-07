use super::html_scraper::HtmlPathsScraper;
use super::javascript_scraper::JsPathsScraper;
use crate::discoverer::http::{ContentType, Response, Url};

use log::warn;

pub struct ResponseScraper {
    javascript_scraper: JsPathsScraper,
    html_scraper: HtmlPathsScraper,
}

impl<'r, 't> ResponseScraper {
    pub fn new() -> Self {
        return Self {
            javascript_scraper: JsPathsScraper::new(),
            html_scraper: HtmlPathsScraper::new(),
        };
    }

    pub fn scrap(&self, response: &Response) -> impl Iterator<Item = Url> {
        let paths = self.scrap_paths(response);
        return UrlCombinator::new(response.url().clone(), paths.into_iter());
    }

    fn scrap_paths(&self, response: &Response) -> Vec<String> {
        if let Some(content_type) = response.content_type() {
            match content_type {
                ContentType::Html => return self.scrap_html(response),
                ContentType::Javascript => {
                    return self.scrap_javascript(response)
                }
            }
        }
        return Vec::new();
    }

    fn scrap_javascript(&self, response: &Response) -> Vec<String> {
        let response_body = response.body();
        return self.javascript_scraper.scrap(response_body).collect();
    }

    fn scrap_html(&self, response: &Response) -> Vec<String> {
        return self.html_scraper.scrap(response.body());
    }
}

struct UrlCombinator<T>
where
    T: Iterator<Item = String>,
{
    url: Url,
    paths: T,
    created_urls: Vec<Url>,
}

impl<T> UrlCombinator<T>
where
    T: Iterator<Item = String>,
{
    pub fn new(url: Url, paths: T) -> Self {
        return Self {
            url,
            paths,
            created_urls: Vec::new(),
        };
    }
}

impl<T> Iterator for UrlCombinator<T>
where
    T: Iterator<Item = String>,
{
    type Item = Url;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let path = self.paths.next()?;

            match self.url.join(&path) {
                Ok(mut url) => {
                    url.set_fragment(None);
                    url.set_query(None);

                    if self.created_urls.contains(&url) {
                        continue;
                    }
                    self.created_urls.push(url.clone());

                    return Some(url);
                }
                Err(error) => {
                    warn!(
                        "Error joining url error({}) url({}) path({})",
                        error, self.url, path
                    );
                }
            }
        }
    }
}
