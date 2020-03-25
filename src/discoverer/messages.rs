use crossbeam_channel::*;
use reqwest::{Response, Url};

pub type ResponseReceiver = Receiver<ResponseMessage>;
pub type ResponseSender = Sender<ResponseMessage>;

pub fn new_response_channel() -> (ResponseSender, ResponseReceiver) {
    return crossbeam_channel::unbounded::<ResponseMessage>();
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

pub type UrlReceiver = Receiver<UrlMessage>;
pub type UrlSender = Sender<UrlMessage>;

pub fn new_url_channel(size: usize) -> (UrlSender, UrlReceiver) {
    return crossbeam_channel::bounded::<UrlMessage>(size);
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

pub type UrlsReceiver = Receiver<UrlsMessage>;
pub type UrlsSender = Sender<UrlsMessage>;

pub fn new_urls_channel() -> (UrlsSender, UrlsReceiver) {
    return crossbeam_channel::unbounded::<UrlsMessage>();
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
