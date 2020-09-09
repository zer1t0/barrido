use super::{Verificator, VerificatorResult, VerificatorTrait};
use crate::http::Response;

pub struct OrVerificator {
    sub_verificators: Vec<Verificator>,
}

impl OrVerificator {
    pub fn new(sub_verificators: Vec<Verificator>) -> Verificator {
        return Box::new(Self { sub_verificators });
    }
}

impl VerificatorTrait for OrVerificator {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult {
        let mut errors = Vec::new();
        for verificator in self.sub_verificators.iter() {
            match verificator.is_valid_response(response) {
                Ok(()) => return Ok(()),
                Err(err) => errors.push(err),
            }
        }
        return Err(format!("{}", errors.join(" & ")));
    }

    fn condition_desc(&self) -> String {
        return format!(
            "Or: {}",
            self.sub_verificators
                .iter()
                .map(|v| v.condition_desc())
                .collect::<Vec<String>>()
                .join(" | ")
        );
    }
}
