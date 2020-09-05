use reqwest;
use reqwest::Url;
use super::channel::{Channel, Sender, Receiver};
use crate::http::Response;

pub type ResultReceiver = Receiver<Result<Answer, Error>>;
pub type ResultSender = Sender<Result<Answer, Error>>;
pub type ResultChannel = Channel<Result<Answer, Error>>;

#[derive(Debug)]
pub struct Answer {
    pub valid: bool,
    pub url: Url,
    pub status: u16,
    pub size: usize,
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

