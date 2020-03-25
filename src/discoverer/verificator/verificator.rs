use super::super::response::Response;
use super::{AndVerificator, NorVerificator};
use std::ops::{BitAnd, Not};

pub type Verificator = Box<dyn VerificatorTrait>;

pub trait VerificatorTrait: Sync + Send {
    fn is_valid_response(&self, response: &Response) -> bool;
}

impl BitAnd for Verificator {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return AndVerificator::new(self, rhs);
    }
}

impl Not for Verificator {
    type Output = Self;

    fn not(self) -> Verificator {
        return NorVerificator::new(self);
    }
}
