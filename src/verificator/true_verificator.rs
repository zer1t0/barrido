use crate::http::Response;
use super::{Verificator, VerificatorTrait, VerificatorResult};

pub struct TrueVerificator {}

impl TrueVerificator {
    pub fn new() -> Verificator {
        return Box::new(Self {});
    }
}

impl VerificatorTrait for TrueVerificator {
    fn is_valid_response(&self, _: &Response) -> VerificatorResult {
        return Ok(());
    }

    fn condition_desc(&self) -> String {
        return "True".to_string();
    }
}
