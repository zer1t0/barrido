use crate::discoverer::http::Response;
use super::{Verificator, VerificatorTrait};

pub struct AndVerificator {
    sub_verificators: Vec<Verificator>,
}

impl AndVerificator {
    pub fn new(sub_verificators: Vec<Verificator>) -> Verificator {
        return Box::new(Self { sub_verificators });
    }
}

impl VerificatorTrait for AndVerificator {
    fn is_valid_response(&self, response: &Response) -> bool {
        for verificator in self.sub_verificators.iter() {
            if !verificator.is_valid_response(response) {
                return false;
            }
        }
        return true;
    }
}
