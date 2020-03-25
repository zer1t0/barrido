use crate::discoverer::http::Response;
use super::{AndVerificator, NorVerificator, OrVerificator};
use std::ops::{BitAnd, Not, BitOr};

pub type Verificator = Box<dyn VerificatorTrait>;

pub trait VerificatorTrait: Sync + Send {
    fn is_valid_response(&self, response: &Response) -> bool;
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
        return NorVerificator::new(self);
    }
}
