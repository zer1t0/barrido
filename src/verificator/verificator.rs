use crate::http::Response;
use super::{AndVerificator, NotVerificator, OrVerificator};
use std::ops::{BitAnd, Not, BitOr};

pub type Verificator = Box<dyn VerificatorTrait>;
pub type VerificatorError = String;
pub type VerificatorResult = Result<(), VerificatorError>;

pub trait VerificatorTrait: Sync + Send {
    fn is_valid_response(&self, response: &Response) -> VerificatorResult;
    fn condition_desc(&self) -> String;
}

impl BitAnd for Verificator {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return AndVerificator::new(vec![self, rhs]);
    }
}

impl BitOr for Verificator {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        return OrVerificator::new(vec![self, rhs]);
    }
}


impl Not for Verificator {
    type Output = Self;

    fn not(self) -> Verificator {
        return NotVerificator::new(self);
    }
}
