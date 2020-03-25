use super::super::response::Response;
use super::{Verificator, VerificatorTrait};

pub struct CodesVerificator {
    codes: Vec<u16>,
}

impl CodesVerificator {
    pub fn new(codes: Vec<u16>) -> Verificator {
        return Box::new(Self { codes });
    }
}

impl VerificatorTrait for CodesVerificator {
    fn is_valid_response(&self, response: &Response) -> bool {
        return self.codes.contains(&response.status());
    }
}
