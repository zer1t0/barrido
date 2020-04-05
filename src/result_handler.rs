use log::info;
use std::time::{Duration, Instant};

use crate::discoverer::communication::result_channel::{ResultReceiver, Answer, Error};
use crate::printer::Printer;

pub struct ResultHandler {
    result_receiver: ResultReceiver,
    end_receiver: crossbeam_channel::Receiver<()>,
    signal_receiver: crossbeam_channel::Receiver<()>,
    progress_scheluder: crossbeam_channel::Receiver<Instant>,
    received_count: usize,
    max_requests_count: usize,
    results: Vec<Answer>,
    printer: Printer,
}

impl ResultHandler {
    pub fn start(
        result_receiver: ResultReceiver,
        end_receiver: crossbeam_channel::Receiver<()>,
        signal_receiver: crossbeam_channel::Receiver<()>,
        max_requests_count: usize,
        printer: Printer,
    ) -> Vec<Answer> {
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
                recv(self.result_receiver) -> result => {
                    info!("valid response received");
                    match result {
                        Ok(ok_result) => self.handle_result(ok_result),
                        _ => {}
                    }
                }
            }
        }

        info!("finishing");
        self.printer.print_clean();
    }

    fn handle_result(&mut self, result: Result<Answer, Error>) {
        self.received_count += 1;
        match result {
            Ok(answer) => self.handle_answer(answer),
            Err(error) => self.handle_error(error)
        }
    }

    fn handle_answer(&mut self, answer: Answer) {
        self.received_count += 1;
        match answer.valid() {
            true => self.handle_valid(answer),
            false => {}
        }
        
    }

    fn handle_valid(&mut self, answer: Answer) {
        self.printer.print_path(
            answer.url(),
            *answer.status(),
            *answer.size(),
        );

        self.results.push(answer);
    }

    fn handle_error(&mut self, error: Error) {
        self.printer.print_error(error);
    }
}
