use super::{Verificator, VerificatorResult, VerificatorTrait};
use crate::http::Response;

pub struct CodesVerificator {
    codes: Vec<u16>,
}

impl CodesVerificator {
    pub fn new(codes: Vec<u16>) -> Verificator {
        return Box::new(Self { codes });
    }
}

impl VerificatorTrait for CodesVerificator {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        match self.codes.contains(&response.status()) {
            true => Ok(()),
            false => Err(format!(
                "Response code {} not in valid codes {:?}",
                response.status(),
                self.codes
            )),
        }
    }

    fn condition_desc(&self) -> String {
        return format!("Codes: {:?}", self.codes);
    }
}
