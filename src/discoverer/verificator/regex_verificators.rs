use super::*;
use regex::Regex;
use super::super::response::Response;

pub struct RegexVerificator {
    regex: Regex
}

impl RegexVerificator {

    pub fn new(regex: Regex) -> Verificator {
        return Box::new(Self{regex});
    }

}

impl VerificatorTrait for RegexVerificator {

    fn is_valid_response(&self, response: &Response) -> bool {
        return self.regex.is_match(response.body());
    }

}
