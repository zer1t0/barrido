use crate::http::Response;
use super::{Verificator, VerificatorTrait};

pub struct NorVerificator {
    verificator_1: Verificator,
}

impl NorVerificator {
    pub fn new(verificator_1: Verificator) -> Verificator {
        return Box::new(Self { verificator_1 });
    }
}

impl VerificatorTrait for NorVerificator {
    fn is_valid_response(&self, response: &Response) -> bool {
        return !self.verificator_1.is_valid_response(response);
    }
}
