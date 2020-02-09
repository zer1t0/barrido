mod verificator;
mod and_verificator;
mod nor_verificator;
mod codes_verificators;
mod regex_verificators;
mod size_verificator;

pub use verificator::*;
pub use and_verificator::*;
pub use nor_verificator::*;
pub use codes_verificators::*;
pub use regex_verificators::*;
pub use size_verificator::*;



pub fn create_default() -> Verificator {
    return  CodesVerificator::new(
        vec![200,204,301,302,307,401,403]
    );
}
