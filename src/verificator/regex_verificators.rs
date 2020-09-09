use crate::http::Response;
use regex::Regex;
use super::{Verificator, VerificatorTrait, VerificatorResult};

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
            false => Err(format!("No regex {} matched by body", self.regex))
        }
    }

    fn condition_desc(&self) -> String {
        return format!("Body regex: {}", self.regex);
    }
}
