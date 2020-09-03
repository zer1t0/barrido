use regex::{CaptureMatches, Regex};

#[derive(Clone)]
pub struct JsPathsScraper {
    paths_regex: Regex,
}

impl<'r, 't> JsPathsScraper {
    pub fn new() -> Self {
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

    pub fn scrap(&'r self, js_body: &'t str) -> JsPathsScraperIter<'r, 't> {
        return JsPathsScraperIter::new(
            self.paths_regex.captures_iter(js_body),
        );
    }
}

pub struct JsPathsScraperIter<'r, 't> {
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
