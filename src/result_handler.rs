use log::info;
use std::time::{Duration, Instant};

use crate::discoverer::communication::{ResultReceiver, ResponseInfo};
use crate::printer::Printer;

pub struct ResultHandler {
    result_receiver: ResultReceiver,
    end_receiver: crossbeam_channel::Receiver<()>,
    signal_receiver: crossbeam_channel::Receiver<()>,
    progress_scheluder: crossbeam_channel::Receiver<Instant>,
    received_count: usize,
    max_requests_count: usize,
    results: Vec<ResponseInfo>,
    printer: Printer,
}

impl ResultHandler {
    pub fn start(
        result_receiver: ResultReceiver,
        end_receiver: crossbeam_channel::Receiver<()>,
        signal_receiver: crossbeam_channel::Receiver<()>,
        max_requests_count: usize,
        printer: Printer,
    ) -> Vec<ResponseInfo> {
        let mut handler = Self::new(
            result_receiver,
            end_receiver,
            signal_receiver,
            max_requests_count,
            printer,
        );

        handler.run();

        return handler.results;
    }

    fn new(
        result_receiver: ResultReceiver,
        end_receiver: crossbeam_channel::Receiver<()>,
        signal_receiver: crossbeam_channel::Receiver<()>,
        max_requests_count: usize,
        printer: Printer,
    ) -> Self {
        return Self {
            result_receiver,
            end_receiver,
            signal_receiver,
            progress_scheluder: crossbeam_channel::tick(Duration::from_millis(
                5,
            )),
            max_requests_count,
            received_count: 0,
            results: Vec::new(),
            printer,
        };
    }

    fn run(&mut self) {
        loop {
            crossbeam_channel::select! {
                recv(self.progress_scheluder) -> _ => {
                    self.printer.print_progress(
                        self.received_count,
                        self.max_requests_count
                    );
                }
                recv(self.end_receiver) -> _ => {
                    info!("end received");
                    break;
                }
                recv(self.signal_receiver) -> _ => {
                    info!("signal received");
                    break;
                }
                recv(self.result_receiver.valid_responses()) -> result => {
                    info!("valid response received");
                    match result {
                        Ok(ok_result) => self.handle_result(ok_result),
                        _ => {}
                    }
                }
                recv(self.result_receiver.invalid_responses()) -> result => {
                    info!("invalid response received");
                    match result {
                        Ok(_) => self.handle_invalid_response(),
                        _ => {}
                    }
                }
                recv(self.result_receiver.errors()) -> error => {
                    info!("error received");
                    match error {
                        Ok(request_error) => self.handle_error(request_error),
                        _ => {}
                    }
                }
            }
        }

        info!("finishing");
        self.printer.print_clean();
    }

    fn handle_result(&mut self, response_info: ResponseInfo) {
        self.printer.print_path(
            response_info.url(),
            response_info.status(),
            response_info.body_length(),
        );

        self.results.push(response_info);
        self.received_count += 1;
    }

    fn handle_invalid_response(&mut self) {
        self.received_count += 1;
    }

    fn handle_error(&mut self, error: reqwest::Error) {
        self.printer.print_error(error);
        self.received_count += 1;
    }
}
