use super::channel::{Channel, Receiver, Sender};
use reqwest::{Response, Url};

pub type ResponseReceiver = Receiver<ResponseMessage>;
pub type ResponseSender = Sender<ResponseMessage>;
pub type ResponseChannel = Channel<ResponseMessage>;

#[derive(Debug)]
pub struct ResponseMessage {
    pub base_url: Url,
    pub response: reqwest::Result<Response>,
}

impl ResponseMessage {
    pub fn new(base_url: Url, response: reqwest::Result<Response>) -> Self {
        return Self { base_url, response };
    }
}
