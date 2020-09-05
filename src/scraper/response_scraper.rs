use super::html_scraper::HtmlPathsScraper;
use super::javascript_scraper::JsPathsScraper;
use crate::http::{ContentType, Response, Url};
use super::url_combinator::UrlCombinator;

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


