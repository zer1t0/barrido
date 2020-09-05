use reqwest::Url;
use super::channel::{Channel, Receiver, Sender};

pub type UrlReceiver = Receiver<UrlMessage>;
pub type UrlSender = Sender<UrlMessage>;
pub type UrlChannel = Channel<UrlMessage>;

pub struct UrlMessage {
    pub base_url: Url,
    pub url: Url,
}

impl UrlMessage {
    pub fn new(base_url: Url, url: Url) -> Self {
        return Self { base_url, url };
    }
}
