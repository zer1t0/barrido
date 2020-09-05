use crate::communication::{
    ResponseMessage, ResponseSender, UrlMessage, UrlReceiver, WaitMutex,
};
use crossbeam_channel::RecvError;
use reqwest::{Client, Url};
use std::sync::Arc;

use log::{error, info, trace, debug};

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
        info!("{} Init", self.id);
        loop {
            match self.wait_for_path() {
                Ok(url_message) => self.get_and_send(url_message),
                Err(_) => {
                    info!("{} Url channel was closed", self.id);
                    break;
                }
            }
        }
        info!("{} Finish", self.id);
    }

    fn get_and_send(&self, url_message: UrlMessage) {
        let response = self.get(url_message.url);

        self.send_response(ResponseMessage::new(
            url_message.base_url,
            response,
        ));
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
        trace!("{} Waiting for url", self.id);
        return self.url_receiver.recv();
    }

    fn send_response(&self, response_message: ResponseMessage) {
        debug!("Send response {:?}", response_message);
        if let Err(error) = self.response_sender.send(response_message) {
            error!("Error sending response {:?}", error);
        }
    }
}
