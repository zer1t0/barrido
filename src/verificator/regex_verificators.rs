use super::{Verificator, VerificatorResult, VerificatorTrait};
use crate::http::Response;
use regex::Regex;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

pub struct BodyRegexVerificator {
    regex: Regex,
}

impl BodyRegexVerificator {
    pub fn new(regex: Regex) -> Verificator {
        return Box::new(Self { regex });
    }
}

impl VerificatorTrait for BodyRegexVerificator {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        match self.regex.is_match(response.body()) {
            true => Ok(()),
            false => Err(format!("No regex {} matched by body", self.regex)),
        }
    }

    fn condition_desc(&self) -> String {
        return format!("Body regex: {}", self.regex);
    }
}

pub struct HeaderRegexVerificator {
    name_regex: Regex,
    value_regex: Regex,
}

impl HeaderRegexVerificator {
    pub fn new(name_regex: Regex, value_regex: Regex) -> Verificator {
        return Box::new(Self {
            name_regex,
            value_regex,
        });
    }

    fn match_name(&self, name: &HeaderName) -> bool {
        return self.name_regex.is_match(&name.to_string());
    }

    fn match_value(&self, value: &HeaderValue) -> bool {
        match value.to_str() {
            Ok(value_str) => return self.value_regex.is_match(value_str),
            Err(_) => return false,
        }
    }
}

impl VerificatorTrait for HeaderRegexVerificator {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        for (name, value) in response.headers().iter() {
            if self.match_name(name) && self.match_value(value) {
                return Ok(());
            }
        }
        return Err(format!(
            "No matched header regex: {}: {}",
            self.name_regex, self.value_regex
        ));
    }

    fn condition_desc(&self) -> String {
        return format!(
            "Header regex: {}: {}",
            self.name_regex, self.value_regex
        );
    }
}
