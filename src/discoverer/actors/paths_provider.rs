use crate::discoverer::communication::{
    UrlMessage, UrlSender, UrlsMessage, UrlsReceiver, WaitMutex,
};
use crossbeam_channel::{TryRecvError, RecvError};
use reqwest::Url;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use log::{info,trace};

pub struct PathProvider {
    url_sender: UrlSender,
    scraper_urls_receiver: UrlsReceiver,
    dispatched_paths: HashMap<String, ()>,
    wait_mutex: WaitMutex,
}

impl PathProvider {
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

    pub fn run(
        self,
        base_urls: Vec<Url>,
        paths_reader: BufReader<File>,
    ) {
        info!("Init");
        self.receive_from_file_and_scraper(&base_urls, paths_reader.lines());
        info!("Finish");
    }

    fn receive_from_file_and_scraper(
        mut self,
        base_urls: &[Url],
        mut file_paths: Lines<BufReader<File>>,
    ) {
        loop {
            match file_paths.next() {
                Some(line) => {
                    let path = line.expect("PathDiscoverer: error unwrapping line");
                    for base_url in base_urls.iter() {
                        self.send_path(base_url, &path);
                        if let Err(_) = self.try_receive_from_scraper() {
                            return self
                                .receive_only_from_file(base_urls, file_paths);
                        }
                    }
                }
                None => {
                    info!("Paths in file finished");
                    return self.receive_only_from_scraper();
                }
            }
        }
    }

    fn try_receive_from_scraper(&mut self) -> Result<(), ()> {
        match self.try_recv_scraper() {
            Ok(paths) => self.send_urls(paths),
            Err(channel_error) => match channel_error {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => return Err(()),
            },
        }
        return Ok(());
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

    fn try_recv_scraper(&self) -> Result<UrlsMessage, TryRecvError> {
        return self.scraper_urls_receiver.try_recv();
    }

    fn receive_only_from_file(
        mut self,
        base_urls: &[Url],
        file_paths: Lines<BufReader<File>>,
    ) {
        for line in file_paths {
            let path = line.unwrap();
            for base_url in base_urls.iter() {
                self.send_path(base_url, &path);
            }
        }
        info!("Paths in file finished");
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
