use crossbeam_channel::*;
use std::sync::*;

use crate::discoverer::communication::{ResultSender, ResponseReceiver, ResponseMessage, WaitMutex, ResponseInfo};
use crate::discoverer::http::Response;
use crate::discoverer::scraper::ScraperManager;
use crate::discoverer::verificator::Verificator;
use reqwest::Url;

use log::{info, debug, trace};

pub struct ResponseHandler {
    response_receiver: ResponseReceiver,
    result_sender: ResultSender,
    verificator: Arc<Verificator>,
    scraper: Box<dyn ScraperManager>,
    wait_mutex: WaitMutex,
    id: usize,
}

impl ResponseHandler {
    pub fn new(
        response_receiver: Receiver<ResponseMessage>,
        result_sender: ResultSender,
        verificator: Arc<Verificator>,
        scraper: Box<dyn ScraperManager>,
        wait_mutex: WaitMutex,
        id: usize,
    ) -> Self {
        return ResponseHandler {
            response_receiver,
            result_sender,
            verificator,
            scraper,
            wait_mutex,
            id,
        };
    }

    pub fn run(&self) {
        info!("{} Init", self.id);
        loop {
            match self.recv() {
                Ok(result) => self.handle_http_result(result),
                Err(_) => {
                    info!("{} Response channel was closed", self.id);
                    break;
                }
            }
        }
        info!("{} Finish", self.id);
    }

    fn recv(&self) -> Result<ResponseMessage, RecvError> {
        let mut is_waiting = self
            .wait_mutex
            .lock()
            .expect("ResponseHandler: error locking wait mutex");
        *is_waiting = true;
        trace!("{} Waiting for response", self.id);
        return self.response_receiver.recv();
    }

    fn handle_http_result(&self, message: ResponseMessage) {
        let base_url = message.base_url;
        match message.response {
            Ok(response) => self.process_response(base_url, response),
            Err(err) => self.send_error(err),
        }
    }

    fn process_response(&self, base_url: Url, response: reqwest::Response) {
        info!("Process response for {}", response.url());
        let response = Response::from(response);
        if self.is_valid_response(&response) {
            self.process_valid_response(base_url, response);
        } else {
            self.process_invalid_response(response);
        }
    }

    fn is_valid_response(&self, response: &Response) -> bool {
        return self.verificator.is_valid_response(response);
    }

    fn process_valid_response(&self, base_url: Url, response: Response) {
        info!("{}: valid response for {}", self.id, response.url());

        self.scrap(base_url, &response);
        self.send_valid(ResponseInfo::new(response));
    }

    fn process_invalid_response(&self, response: Response) {
        info!("{}: invalid response for {}", self.id, response.url());
        self.send_invalid(ResponseInfo::new(response));
    }

    fn send_error(&self, err: reqwest::Error) {
        debug!("Send error: {:?}", err);
        self.result_sender.send_error(err);
    }

    fn send_invalid(&self, response_info: ResponseInfo){
        debug!("Send invalid response: {:?}", response_info);
        self.result_sender.send_invalid_response(response_info);
    }

    fn send_valid(&self, response_info: ResponseInfo) {
        debug!("Send valid response: {:?}", response_info);
        self.result_sender.send_valid_response(response_info);
    }

    fn scrap(&self, base_url: Url, response: &Response) {
        self.scraper.scrap_response(base_url, response);
    }
}
