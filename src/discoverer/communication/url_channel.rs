use crossbeam_channel::{bounded, Receiver, Sender};
use getset::Getters;
use reqwest::Url;

pub type UrlReceiver = Receiver<UrlMessage>;
pub type UrlSender = Sender<UrlMessage>;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct UrlChannel {
    sender: UrlSender,
    receiver: UrlReceiver,
}

impl UrlChannel {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = bounded::<UrlMessage>(size);
        return Self { sender, receiver };
    }
}

pub struct UrlMessage {
    pub base_url: Url,
    pub url: Url,
}

impl UrlMessage {
    pub fn new(base_url: Url, url: Url) -> Self {
        return Self { base_url, url };
    }
}
