use crossbeam_channel::{Receiver, Sender, unbounded};
use getset::Getters;
use reqwest::{Response, Url};

pub type ResponseReceiver = Receiver<ResponseMessage>;
pub type ResponseSender = Sender<ResponseMessage>;

#[derive(Getters)]
#[getset (get = "pub")]
pub struct ResponseChannel {
    sender: ResponseSender,
    receiver: ResponseReceiver
}

impl Default for ResponseChannel {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<ResponseMessage>();
        return Self {sender, receiver};
    }
}

pub struct ResponseMessage {
    pub base_url: Url,
    pub response: reqwest::Result<Response>,
}

impl ResponseMessage {
    pub fn new(base_url: Url, response: reqwest::Result<Response>) -> Self {
        return Self { base_url, response };
    }
}
