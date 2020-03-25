use super::super::response::Response;
use super::{Verificator, VerificatorTrait};

pub struct OrVerificator {
    sub_verificators: Vec<Verificator>,
}

impl OrVerificator {
    pub fn new(sub_verificators: Vec<Verificator>) -> Verificator {
        return Box::new(Self { sub_verificators });
    }
}

impl VerificatorTrait for OrVerificator {
    fn is_valid_response(&self, response: &Response) -> bool {
        for verificator in self.sub_verificators.iter() {
            if verificator.is_valid_response(response) {
                return true;
            }
        }
        return false;
    }
}
