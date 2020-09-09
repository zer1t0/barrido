use crate::http::Response;
use super::{Verificator, VerificatorTrait, VerificatorResult};

pub struct NotVerificator {
    verificator_1: Verificator,
}

impl NotVerificator {
    pub fn new(verificator_1: Verificator) -> Verificator {
        return Box::new(Self { verificator_1 });
    }
}

impl VerificatorTrait for NotVerificator {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        match self.verificator_1.is_valid_response(response) {
            Ok(()) => Err(format!("Not {}", self.condition_desc())),
            Err(_) => Ok(())
        }
    }

    fn condition_desc(&self) -> String {
        return format!("Not: {}", self.verificator_1.condition_desc());
    }
}
