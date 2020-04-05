use crossbeam_channel::*;
use std::sync::*;

use crate::discoverer::communication::result_channel::{
    Answer, Error, ResultSender,
};
use crate::discoverer::communication::{
    ResponseMessage, ResponseReceiver, WaitMutex,
};
use crate::discoverer::http::Response;
use crate::discoverer::scraper::ScraperManager;
use crate::discoverer::verificator::Verificator;
use reqwest::Url;

use log::{debug, info, trace};

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
        let request_url = message.request_url;
        match message.response {
            Ok(response) => {
                self.process_response(base_url, request_url, response)
            }
            Err(err) => self.send_error(err),
        }
    }

    fn process_response(
        &self,
        base_url: Url,
        request_url: Url,
        response: reqwest::Response,
    ) {
        info!("Process response for {}", response.url());
        let response = Response::from(response);
        if self.is_valid(&response) {
            self.process_valid(base_url, request_url, response);
        } else {
            self.process_invalid(request_url, response);
        }
    }

    fn is_valid(&self, response: &Response) -> bool {
        return self.verificator.is_valid_response(response);
    }

    fn process_valid(
        &self,
        base_url: Url,
        request_url: Url,
        response: Response,
    ) {
        info!("{}: valid response for {}", self.id, response.url());
        self.scrap(base_url, &response);

        let answer = Answer::new_valid(request_url, response);
        self.send_answer(answer);
    }

    fn process_invalid(&self, url: Url, response: Response) {
        info!("{}: invalid response for {}", self.id, response.url());
        let answer = Answer::new_invalid(url, response);
        self.send_answer(answer);
    }

    fn send_error(&self, err: Error) {
        debug!("Send error: {:?}", err);
        self.send(Err(err));
    }

    fn send_answer(&self, answer: Answer) {
        debug!("Send answer: {:?}", answer);
        self.send(Ok(answer));
    }

    fn send(&self, result: Result<Answer, Error>) {
        if let Err(error) = self.result_sender.send(result) {
            panic!("Error sending result: {:?}", error);
        }
    }

    fn scrap(&self, base_url: Url, response: &Response) {
        self.scraper.scrap_response(base_url, response);
    }
}
