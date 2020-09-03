use crate::discoverer::communication::{Channel, Receiver};
use crate::discoverer::http::Url;

pub struct UrlPathProvider {
    urls: Vec<Url>,
    paths: Vec<String>,
    channel: Channel<(Url, Url)>,
}

impl UrlPathProvider {
    pub fn new(urls: Vec<Url>, paths: Vec<String>) -> Self {
        return Self {
            urls,
            paths,
            channel: Channel::with_capacity(20),
        };
    }

    pub fn combine(self) {
        for path in self.paths.iter() {
            for base_url in self.urls.iter() {
                let url =
                    base_url.join(&path).expect("error combining path and url");
                if let Err(error) =
                    self.channel.sender.send((base_url.clone(), url))
                {
                    panic!("Error sending url to aggregator {:?}", error);
                }
            }
        }
    }

    pub fn receiver(&self) -> &Receiver<(Url, Url)> {
        return &self.channel.receiver;
    }
}
