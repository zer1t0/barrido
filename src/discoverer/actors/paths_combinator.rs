use crate::discoverer::http::Url;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::discoverer::communication::{Channel, Receiver};
use super::url_provider::UrlProvider;

pub struct PathCombinatorProvider {
    urls: Vec<Url>,
    paths: Lines<BufReader<File>>,
    channel: Channel<Url>,
}

impl PathCombinatorProvider {
    pub fn new(urls: Vec<Url>, paths: Lines<BufReader<File>>) -> Self {
        return Self {urls, paths, channel: Channel::new(20)};
    }

    pub fn combine(self) {
        for line in self.paths {
            let path = line.expect("error unwrapping line");
            for url in self.urls.iter() {
                let url = url.join(&path).expect("error combining path and url");                
                if let Err(error) = self.channel.sender().send(url) {
                    panic!("Error sending url to aggregator {:?}", error);
                }
            }
        }

    }

}

impl UrlProvider for PathCombinatorProvider {
    fn receiver(&self) -> &Receiver<Url> {
        return self.channel.receiver();
    }
}
