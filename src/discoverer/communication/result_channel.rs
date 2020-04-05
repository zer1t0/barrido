use reqwest;
use reqwest::Url;
use super::channel::{Channel, Sender, Receiver};
use crate::discoverer::http::Response;
use getset::Getters;

pub type ResultReceiver = Receiver<Result<Answer, Error>>;
pub type ResultSender = Sender<Result<Answer, Error>>;
pub type ResultChannel = Channel<Result<Answer, Error>>;

#[derive(Debug, Getters)]
#[getset (get = "pub")]
pub struct Answer {
    valid: bool,
    url: Url,
    status: u16,
    size: usize,
}


impl Answer {
    pub fn new(valid: bool, response: Response) -> Self {
        return Self {
            valid,
            url: response.url().clone(),
            status: response.status(),
            size: response.body().len()
        }
    }

    pub fn new_valid(response: Response) -> Self {
        return Self::new(true, response);
    }

    pub fn new_invalid(response: Response) -> Self {
        return Self::new(false, response);
    }
}

pub type Error = reqwest::Error;

