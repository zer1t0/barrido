use crate::http::Response;
use super::{Verificator, VerificatorTrait, VerificatorResult};

pub struct AndVerificator {
    sub_verificators: Vec<Verificator>,
}

impl AndVerificator {
    pub fn new(sub_verificators: Vec<Verificator>) -> Verificator {
        return Box::new(Self { sub_verificators });
    }
}

impl VerificatorTrait for AndVerificator {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        for verificator in self.sub_verificators.iter() {
            let _ = verificator.is_valid_response(response)?;
        }
        return Ok(());
    }

    fn condition_desc(&self) -> String {
        return format!(
            "And: {}",
            self.sub_verificators
                .iter()
                .map(|v| v.condition_desc())
                .collect::<Vec<String>>()
                .join(" & ")
        );
    }
}
