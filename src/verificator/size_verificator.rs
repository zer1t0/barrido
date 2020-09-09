use super::{Verificator, VerificatorResult, VerificatorTrait};
use crate::http::Response;

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
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        let size = response.body().len();
        if self.min_size <= size && size <= self.max_size {
            return Ok(());
        }

        return Err(format!(
            "{} size is not in range {}-{}",
            size, self.min_size, self.max_size
        ));
    }

    fn condition_desc(&self) -> String {
        return format!("Size in range {}-{}", self.min_size, self.max_size);
    }
}
