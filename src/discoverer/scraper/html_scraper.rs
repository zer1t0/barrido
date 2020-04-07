use scraper::{Html, Selector};

struct ScrappedTag {
    name: &'static str,
    attrs: Vec<&'static str>,
}

pub struct HtmlPathsScraper {
    tags_to_scrape: Vec<ScrappedTag>,
}

impl HtmlPathsScraper {
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

    pub fn scrap(&self, response_body: &str) -> Vec<String> {
        let html_body = Html::parse_document(response_body);
        let mut paths = Vec::new();

        for scraped_tag in self.tags_to_scrape.iter() {
            let selector = Selector::parse(scraped_tag.name).unwrap();

            for tag in html_body.select(&selector) {
                let tag_paths = self.extract_paths_from_tag_attributes(
                    &tag,
                    &scraped_tag.attrs,
                );
                paths.extend(tag_paths);
            }
        }

        return paths;
    }

    fn extract_paths_from_tag_attributes(
        &self,
        script_tag: &scraper::element_ref::ElementRef,
        attrs: &Vec<&'static str>,
    ) -> Vec<String> {
        let mut paths = vec![];

        for attr in attrs.iter() {
            if let Some(path) = script_tag.value().attr(attr) {
                paths.push(path.to_string());
            }
        }

        return paths;
    }
}
