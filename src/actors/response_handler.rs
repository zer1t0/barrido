use crossbeam_channel::*;
use std::sync::*;

use crate::communication::result_channel::{Answer, Error, ResultSender};
use crate::communication::{ResponseMessage, ResponseReceiver, WaitMutex};
use crate::http::Response;
use crate::scraper::ScraperProvider;
use crate::verificator::Verificator;
use reqwest::Url;

use log::{debug, trace};

pub struct ResponseHandler {
    response_receiver: ResponseReceiver,
    result_sender: ResultSender,
    verificator: Arc<Verificator>,
    scraper: Arc<Box<dyn ScraperProvider>>,
    wait_mutex: WaitMutex,
    id: usize,
}

impl ResponseHandler {
    pub fn new(
        response_receiver: Receiver<ResponseMessage>,
        result_sender: ResultSender,
        verificator: Arc<Verificator>,
        scraper: Arc<Box<dyn ScraperProvider>>,
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
        debug!("Responser {}: Init", self.id);
        loop {
            match self.recv() {
                Ok(result) => self.handle_http_result(result),
                Err(_) => {
                    debug!(
                        "Responser {}: Response channel was closed",
                        self.id
                    );
                    break;
                }
            }
        }
        debug!("Responser {}: Finish", self.id);
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
        debug!(
            "Responser {}: Process response for {}",
            self.id,
            response.url()
        );
        let response = Response::from(response);
        if self.is_valid(&response) {
            self.process_valid(base_url, response);
        } else {
            self.process_invalid(response);
        }
    }

    fn is_valid(&self, response: &Response) -> bool {
        match self.verificator.is_valid_response(response) {
            Ok(()) => true,
            Err(err) => {
                debug!(
                    "Response {} not meet verificator condition: {}",
                    response.url(),
                    err
                );
                return false;
            }
        }
    }

    fn process_valid(&self, base_url: Url, response: Response) {
        debug!(
            "Responser {}: valid response for {}",
            self.id,
            response.url()
        );
        self.scrap(base_url, &response);

        let answer = Answer::new_valid(response);
        self.send_answer(answer);
    }

    fn process_invalid(&self, response: Response) {
        debug!(
            "Responser {}: invalid response for {}",
            self.id,
            response.url()
        );
        let answer = Answer::new_invalid(response);
        self.send_answer(answer);
    }

    fn send_error(&self, err: Error) {
        trace!("Responser {}: Send error: {:?}", self.id, err);
        self.send(Err(err));
    }

    fn send_answer(&self, answer: Answer) {
        trace!("Responser {}: Send answer: {:?}", self.id, answer);
        self.send(Ok(answer));
    }

    fn send(&self, result: Result<Answer, Error>) {
        if let Err(error) = self.result_sender.send(result) {
            panic!("Responser {}: Error sending result: {:?}", self.id, error);
        }
    }

    fn scrap(&self, base_url: Url, response: &Response) {
        self.scraper.scrap(base_url, response);
    }
}
