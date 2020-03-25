mod and_verificator;
mod codes_verificators;
mod nor_verificator;
mod or_verificator;
mod regex_verificators;
mod size_verificator;
mod true_verificator;
mod verificator;

pub use and_verificator::AndVerificator;
pub use codes_verificators::CodesVerificator;
pub use nor_verificator::NorVerificator;
pub use or_verificator::OrVerificator;
pub use regex_verificators::RegexVerificator;
pub use size_verificator::SizeVerificator;
pub use true_verificator::TrueVerificator;
pub use verificator::{Verificator, VerificatorTrait};

pub fn create_default() -> Verificator {
    return CodesVerificator::new(vec![200, 204, 301, 302, 307, 401, 403]);
}
