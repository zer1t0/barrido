use crate::http::Response;
use super::{Verificator, VerificatorTrait};

pub struct SizeVerificator {
    min_size: usize,
    max_size: usize,
}

impl SizeVerificator {
    pub fn new_range(min_size: usize, max_size: usize) -> Verificator {
        return Box::new(Self { min_size, max_size });
    }

}

impl VerificatorTrait for SizeVerificator {
    fn is_valid_response(&self, response: &Response) -> bool {
        let size = response.body().len();
        return self.min_size <= size && size <= self.max_size;
    }
}
