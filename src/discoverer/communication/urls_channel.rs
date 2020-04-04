use reqwest::Url;
use super::channel::{Channel, Sender, Receiver};

pub type UrlsReceiver = Receiver<UrlsMessage>;
pub type UrlsSender = Sender<UrlsMessage>;
pub type UrlsChannel = Channel<UrlsMessage>;

pub struct UrlsMessage {
    pub base_url: Url,
    pub urls: Vec<Url>,
}

impl UrlsMessage {
    pub fn new(base_url: Url, urls: Vec<Url>) -> Self {
        return Self { base_url, urls };
    }
}
