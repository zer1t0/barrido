mod and_verificator;
mod codes_verificators;
mod nor_verificator;
mod or_verificator;
mod regex_verificators;
mod size_verificator;
mod verificator;

pub use or_verificator::*;
pub use and_verificator::*;
pub use codes_verificators::*;
pub use nor_verificator::*;
pub use regex_verificators::*;
pub use size_verificator::*;
pub use verificator::*;

pub fn create_default() -> Verificator {
    return CodesVerificator::new(vec![200, 204, 301, 302, 307, 401, 403]);
}
