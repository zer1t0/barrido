use crossbeam_channel::*;
use std::sync::*;

use super::communication::ResultSender;
use super::messages::*;
use super::response::Response;
use super::response_info::ResponseInfo;
use super::scraper::ScraperManager;
use super::verificator::Verificator;
use super::wait_mutex::WaitMutex;
use reqwest::Url;

use log::info;

pub(super) struct ResponseHandler {
    response_receiver: ResponseReceiver,
    result_sender: ResultSender,
    verificator: Arc<Verificator>,
    scraper: Box<dyn ScraperManager>,
    wait_mutex: WaitMutex,
    id: usize,
}

impl ResponseHandler {
    pub(super) fn new(
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

    pub(super) fn run(&self) {
        loop {
            match self.wait_for_response() {
                Ok(result) => self.handle_http_result(result),
                Err(_) => break,
            }
        }
    }

    fn wait_for_response(&self) -> Result<ResponseMessage, RecvError> {
        let mut is_waiting = self
            .wait_mutex
            .lock()
            .expect("Response_Handler: error locking wait mutex");
        *is_waiting = true;
        return self.response_receiver.recv();
    }

    fn handle_http_result(&self, message: ResponseMessage) {
        let base_url = message.base_url;
        match message.response {
            Ok(response) => self.process_response(base_url, response),
            Err(err) => self.result_sender.send_error(err),
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

        self.scraper.scrap_response(base_url, &response);
        let response_info = ResponseInfo::new(response);
        self.result_sender.send_valid_response(response_info);
    }

    fn process_invalid_response(&self, response: Response) {
        info!("{}: invalid response for {}", self.id, response.url());
        let response_info = ResponseInfo::new(response);
        self.result_sender.send_invalid_response(response_info)
    }
}
