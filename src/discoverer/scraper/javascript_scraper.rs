use crate::discoverer::http::Response;
use regex::{CaptureMatches, Regex};
use reqwest::Url;

pub struct JsScraper {
    paths_scraper: JsPathsScraper,
}

impl JsScraper {
    pub fn new() -> Self {
        return Self {
            paths_scraper: JsPathsScraper::new(),
        };
    }

    pub fn scrap(&self, response: &Response) -> Vec<Url> {
        let body = response.body();
        return self.scrap_in_string(body, response.url());
    }

    fn scrap_in_string(&self, js_code: &str, current_url: &Url) -> Vec<Url> {
        let mut urls = Vec::new();

        for path in self.paths_scraper.scrap(&js_code) {
            self.clean_and_add_path(current_url, &mut urls, &path);
        }
        return urls;
    }

    fn clean_and_add_path(
        &self,
        current_url: &Url,
        urls: &mut Vec<Url>,
        path: &str,
    ) {
        match current_url.join(&path) {
            Ok(mut url) => {
                url.set_fragment(None);
                url.set_query(None);
                if !urls.contains(&url) {
                    urls.push(url);
                }
            }
            Err(error) => {
                panic!(
                    "Error joining url error({}) url({}) path({})",
                    error, current_url, path
                );
            }
        }
    }
}

struct JsPathsScraper {
    paths_regex: Regex,
}

impl<'r, 't> JsPathsScraper {
    fn new() -> Self {
        let path_segment_regex = r#"/[\dA-Za-z\-_~\.%]+(/[\dA-Za-z\-_~\.%]*)*"#;
        let query_segment_regex = r#"\?([\dA-Za-z\-_~\.%=&]*)"#;
        let fragment_segment_regex = r#"#([\dA-Za-z\-_~\.%=&]*)"#;
        let base_re = format!(
            r#"['"](({})({})?({})?)['"]"#,
            path_segment_regex, query_segment_regex, fragment_segment_regex
        );

        return Self {
            paths_regex: Regex::new(&base_re).unwrap(),
        };
    }

    fn scrap(&'r self, js_body: &'t str) -> JsPathsScraperIter<'r, 't> {
        return JsPathsScraperIter::new(
            self.paths_regex.captures_iter(js_body),
        );
    }
}

struct JsPathsScraperIter<'r, 't> {
    matches: CaptureMatches<'r, 't>,
}

impl<'r, 't> JsPathsScraperIter<'r, 't> {
    fn new(matches: CaptureMatches<'r, 't>) -> Self {
        return Self { matches };
    }
}

impl<'r, 't> Iterator for JsPathsScraperIter<'r, 't> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let capture = self.matches.next()?;
        return Some(capture.get(1).unwrap().as_str().into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_regex(message: &str) -> Vec<String> {
        let scraper = JsPathsScraper::new();
        return scraper.scrap(message).collect();
    }

    #[test]
    fn double_quote_alphanumeric() {
        assert_eq!(vec!["/aB3.-_~%27"], check_regex("\"/aB3.-_~%27\""));
    }

    #[test]
    fn double_quote_invalid_characters() {
        assert_eq!(vec!["/aB3.-_~%27"], check_regex("\"/aB3.-_~%27\" \"/(\""));
    }

    #[test]
    fn double_quote_multilevel() {
        assert_eq!(
            vec!["/aB3.-_~%27/aB3.-_~%27"],
            check_regex("\"/aB3.-_~%27/aB3.-_~%27\"")
        );
    }

    #[test]
    fn single_quote_multilevel() {
        assert_eq!(vec!["/abc1234/"], check_regex("'/abc1234/'"));
    }

    #[test]
    fn several_valid_strings() {
        assert_eq!(vec!["/abc", "/123"], check_regex("\"/abc\" \"/123\""));
    }

    #[test]
    fn double_quote_with_param() {
        assert_eq!(
            vec!["/aB3.-_~%27?query=a"],
            check_regex("\"/aB3.-_~%27?query=a\"")
        );
    }

    #[test]
    fn double_quote_with_fragment() {
        assert_eq!(vec!["/a#bcd"], check_regex("\"/a#bcd\""));
    }

    #[test]
    fn double_slash() {
        assert_eq!(Vec::<String>::new(), check_regex("\"//\""));
    }

    #[test]
    fn double_slash_after_path() {
        assert_eq!(vec!["/aaa//"], check_regex("\"/aaa//\""));
    }
}
