use super::messages::*;
use super::wait_mutex::WaitMutex;
use crossbeam_channel::RecvError;
use reqwest::{Client, Url};
use std::sync::Arc;

use log::info;

pub struct Requester {
    client: Arc<Client>,
    url_receiver: UrlReceiver,
    response_sender: ResponseSender,
    wait_mutex: WaitMutex,
    id: usize,
}

impl Requester {
    pub fn new(
        client: Arc<Client>,
        url_receiver: UrlReceiver,
        response_sender: ResponseSender,
        wait_mutex: WaitMutex,
        id: usize,
    ) -> Self {
        Self {
            client,
            url_receiver,
            response_sender,
            wait_mutex,
            id,
        }
    }

    pub fn run(&self) {
        loop {
            match self.wait_for_path() {
                Ok(url_message) => self.get_and_send(url_message),
                Err(_) => {
                    info!("Closing requester {}", self.id);
                    break;
                }
            }
        }
    }

    fn get_and_send(&self, url_message: UrlMessage) {
        let response = self.get(url_message.url);
        let response_message =
            ResponseMessage::new(url_message.base_url, response);
        self.send_response(response_message);
    }

    fn get(&self, url: Url) -> reqwest::Result<reqwest::Response> {
        return self.client.get(url).send();
    }

    fn wait_for_path(&self) -> Result<UrlMessage, RecvError> {
        let mut is_waiting = self
            .wait_mutex
            .lock()
            .expect("Requester: error locking wait mutex");

        *is_waiting = true;
        return self.url_receiver.recv();
    }

    fn send_response(&self, response_message: ResponseMessage) {
        if let Err(error) = self.response_sender.send(response_message) {
            println!("Requester: error sending {:?}", error);
        }
    }
}
