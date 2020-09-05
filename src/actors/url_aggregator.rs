use crate::communication::{
    UrlMessage, UrlSender, UrlsMessage, UrlsReceiver, WaitMutex,
};
use crossbeam_channel::RecvError;
use reqwest::Url;
use std::collections::HashMap;

use log::{info, trace};

pub struct UrlAggregator {
    url_sender: UrlSender,
    scraper_urls_receiver: UrlsReceiver,
    dispatched_paths: HashMap<String, ()>,
    wait_mutex: WaitMutex,
}

impl UrlAggregator {
    pub fn new(
        url_sender: UrlSender,
        scraper_urls_receiver: UrlsReceiver,
        wait_mutex: WaitMutex,
    ) -> Self {
        return Self {
            url_sender,
            scraper_urls_receiver,
            dispatched_paths: HashMap::new(),
            wait_mutex,
        };
    }

    pub fn run(self, base_urls: Vec<Url>, paths: Vec<String>) {
        info!("Init");
        self.receive_from_file_and_scraper(&base_urls, paths);
        info!("Finish");
    }

    fn receive_from_file_and_scraper(
        mut self,
        base_urls: &[Url],
        file_paths: Vec<String>,
    ) {
        for path in file_paths {
            for base_url in base_urls {
                self.send_path(base_url, &path);
            }
        }
        return self.receive_only_from_scraper();
    }

    fn receive_only_from_scraper(mut self) {
        loop {
            match self.recv_scraper() {
                Ok(paths) => self.send_urls(paths),
                Err(_) => {
                    info!("No more paths from the scraper");
                    break;
                }
            }
        }
    }

    fn recv_scraper(&self) -> Result<UrlsMessage, RecvError> {
        let _block = self
            .wait_mutex
            .lock()
            .expect("PathProvider: Error locking mutex");
        return self.scraper_urls_receiver.recv();
    }

    fn send_path(&mut self, base_url: &Url, path: &str) {
        self.send_url(base_url, base_url.join(path).unwrap());
    }

    fn send_urls(&mut self, urls_message: UrlsMessage) {
        let base_url = &urls_message.base_url;
        for url in urls_message.urls {
            self.send_url(base_url, url);
        }
    }

    fn send_url(&mut self, base_url: &Url, url: Url) {
        if !self.dispatched_paths.contains_key(url.as_str()) {
            self.dispatched_paths.insert(String::from(url.as_str()), ());

            trace!("Send url {}", url);
            let url_message = UrlMessage::new(base_url.clone(), url);

            self.url_sender
                .send(url_message)
                .expect("PathProvider: error sending url");
        }
    }
}
