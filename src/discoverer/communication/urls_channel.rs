use crossbeam_channel::{unbounded, Receiver, Sender};
use getset::Getters;
use reqwest::Url;

pub type UrlsReceiver = Receiver<UrlsMessage>;
pub type UrlsSender = Sender<UrlsMessage>;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct UrlsChannel {
    sender: UrlsSender,
    receiver: UrlsReceiver,
}

impl Default for UrlsChannel {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<UrlsMessage>();
        return Self { sender, receiver };
    }
}

pub struct UrlsMessage {
    pub base_url: Url,
    pub urls: Vec<Url>,
}

impl UrlsMessage {
    pub fn new(base_url: Url, urls: Vec<Url>) -> Self {
        return Self { base_url, urls };
    }
}
