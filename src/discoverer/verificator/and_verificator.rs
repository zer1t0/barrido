use super::super::response::Response;
use super::{Verificator, VerificatorTrait};

pub struct AndVerificator {
    verificator_1: Verificator,
    verificator_2: Verificator,
}

impl AndVerificator {
    pub fn new(
        verificator_1: Verificator,
        verificator_2: Verificator,
    ) -> Verificator {
        return Box::new(Self {
            verificator_1,
            verificator_2,
        });
    }
}

impl VerificatorTrait for AndVerificator {
    fn is_valid_response(&self, response: &Response) -> bool {
        return self.verificator_1.is_valid_response(response)
            && self.verificator_2.is_valid_response(response);
    }
}
