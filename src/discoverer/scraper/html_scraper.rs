use crate::discoverer::http::Response;
use reqwest::Url;
use scraper::{Html, Selector};

struct ScrappedTag {
    name: &'static str,
    attrs: Vec<&'static str>,
}

pub struct HtmlScraper {
    tags_to_scrape: Vec<ScrappedTag>,
}

impl HtmlScraper {
    pub fn new() -> Self {
        let tags_to_scrape = vec![
            ScrappedTag {
                name: "a",
                attrs: vec!["href"],
            },
            ScrappedTag {
                name: "script",
                attrs: vec!["src", "data-src"],
            },
            ScrappedTag {
                name: "form",
                attrs: vec!["action"],
            },
            ScrappedTag {
                name: "link",
                attrs: vec!["href"],
            },
        ];
        return Self { tags_to_scrape };
    }

    pub fn scrap(&self, response: &Response) -> Vec<Url> {
        let body = response.body();
        return HtmlPageScraper::new(&response.url(), &self.tags_to_scrape)
            .scrap(&body);
    }
}

struct HtmlPageScraper<'a> {
    current_url: &'a Url,
    tags_to_scrape: &'a Vec<ScrappedTag>,
}

impl<'a> HtmlPageScraper<'a> {
    fn new(current_url: &'a Url, tags_to_scrape: &'a Vec<ScrappedTag>) -> Self {
        return Self {
            current_url,
            tags_to_scrape,
        };
    }

    pub(super) fn scrap(&self, response_body: &str) -> Vec<Url> {
        let html_body = Html::parse_document(response_body);

        let mut urls = Vec::new();

        for scraped_tag in self.tags_to_scrape.iter() {
            let selector = Selector::parse(scraped_tag.name).unwrap();

            for tag in html_body.select(&selector) {
                for path in self
                    .extract_paths_from_tag_attributes(&tag, &scraped_tag.attrs)
                {
                    if !urls.contains(&path) {
                        urls.push(path);
                    }
                }
            }
        }

        return urls;
    }

    fn extract_paths_from_tag_attributes(
        &self,
        script_tag: &scraper::element_ref::ElementRef,
        attrs: &Vec<&'static str>,
    ) -> Vec<Url> {
        let mut urls = vec![];

        for attr in attrs.iter() {
            if let Some(link) = script_tag.value().attr(attr) {
                urls.push(self.parse_link_to_url(link));
            }
        }

        return urls;
    }

    fn parse_link_to_url(&self, link: &str) -> Url {
        let mut url = self
            .current_url
            .join(&link)
            .expect("HTML Scraper: error join url");
        url.set_fragment(None);
        url.set_query(None);
        return url;
    }
}
