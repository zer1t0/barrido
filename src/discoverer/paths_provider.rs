use crossbeam_channel::*;
use reqwest::Url;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::discoverer::communication::{UrlSender, UrlsReceiver, UrlMessage, UrlsMessage};
use super::wait_mutex::WaitMutex;

use log::info;

pub(super) struct PathProvider {
    url_sender: UrlSender,
    scraper_urls_receiver: UrlsReceiver,
    dispatched_paths: HashMap<String, ()>,
    wait_mutex: WaitMutex,
}

impl PathProvider {
    pub(super) fn new(
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

    pub(super) fn run(
        self,
        base_urls: Vec<Url>,
        paths_reader: BufReader<std::fs::File>,
    ) {
        self.receive_from_file_and_scraper(&base_urls, paths_reader.lines());
    }

    fn receive_from_file_and_scraper(
        mut self,
        base_urls: &[Url],
        mut file_paths: Lines<BufReader<File>>,
    ) {
        loop {
            match file_paths.next() {
                Some(line) => {
                    let path = line.unwrap();
                    for base_url in base_urls.iter() {
                        self.send_path(base_url, &path);
                        if let Err(_) = self.try_receive_from_scraper() {
                            return self
                                .receive_only_from_file(base_urls, file_paths);
                        }
                    }
                }
                None => return self.receive_only_from_scraper(),
            }
        }
    }

    fn try_receive_from_scraper(&mut self) -> Result<(), ()> {
        match self.scraper_urls_receiver.try_recv() {
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
            match self.wait_for_scraper_paths() {
                Ok(paths) => self.send_urls(paths),
                Err(_) => break,
            }
        }
    }

    fn wait_for_scraper_paths(&self) -> Result<UrlsMessage, RecvError> {
        let _block = self
            .wait_mutex
            .lock()
            .expect("PathProvider: Error locking mutex");
        return self.scraper_urls_receiver.recv();
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

            info!("send {}", url);
            let url_message = UrlMessage::new(base_url.clone(), url);

            self.url_sender
                .send(url_message)
                .expect("PathProvider: error sending url");
        }
    }
}
