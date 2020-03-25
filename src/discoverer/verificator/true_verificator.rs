use crate::discoverer::http::Response;
use super::{Verificator, VerificatorTrait};

pub struct TrueVerificator {}

impl TrueVerificator {
    pub fn new() -> Verificator {
        return Box::new(Self {});
    }
}

impl VerificatorTrait for TrueVerificator {
    fn is_valid_response(&self, _: &Response) -> bool {
        return true;
    }
}
