use crate::http::Response;
use regex::Regex;
use super::{Verificator, VerificatorTrait, VerificatorResult};

pub struct RegexVerificator {
    regex: Regex,
}

impl RegexVerificator {
    pub fn new(regex: Regex) -> Verificator {
        return Box::new(Self { regex });
    }
}

impl VerificatorTrait for RegexVerificator {
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
