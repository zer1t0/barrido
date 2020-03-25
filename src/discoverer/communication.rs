use super::response_info::ResponseInfo;
use crossbeam_channel::*;
use reqwest;

pub struct ResultCommunicator {
    sender: ResultSender,
    receiver: ResultReceiver,
}

impl ResultCommunicator {
    pub fn new() -> Self {
        let (valid_response_sender, valid_response_receiver) =
            unbounded::<ResponseInfo>();
        let (invalid_response_sender, invalid_response_receiver) =
            unbounded::<ResponseInfo>();
        let (error_sender, error_receiver) = unbounded::<reqwest::Error>();

        let sender = ResultSender::new(
            valid_response_sender,
            invalid_response_sender,
            error_sender,
        );

        let receiver = ResultReceiver::new(
            valid_response_receiver,
            invalid_response_receiver,
            error_receiver,
        );
        return Self { sender, receiver };
    }

    pub fn sender(&self) -> &ResultSender {
        return &self.sender;
    }

    pub fn receiver(&self) -> &ResultReceiver {
        return &self.receiver;
    }
}

#[derive(Clone)]
pub struct ResultSender {
    valid_responses: Sender<ResponseInfo>,
    invalid_responses: Sender<ResponseInfo>,
    errors: Sender<reqwest::Error>,
}

impl ResultSender {
    pub fn new(
        valid_responses: Sender<ResponseInfo>,
        invalid_responses: Sender<ResponseInfo>,
        errors: Sender<reqwest::Error>,
    ) -> Self {
        return Self {
            valid_responses,
            invalid_responses,
            errors,
        };
    }

    pub fn send_valid_response(&self, response: ResponseInfo) {
        self.valid_responses
            .send(response)
            .expect("ResultSender: error sending valid response");
    }

    pub fn send_invalid_response(&self, response: ResponseInfo) {
        self.invalid_responses
            .send(response)
            .expect("ResultSender: error sending invalid response");
    }

    pub fn send_error(&self, error: reqwest::Error) {
        self.errors
            .send(error)
            .expect("ResultSender: error sending error");
    }
}

#[derive(Clone)]
pub struct ResultReceiver {
    valid_responses: Receiver<ResponseInfo>,
    invalid_responses: Receiver<ResponseInfo>,
    errors: Receiver<reqwest::Error>,
}

impl ResultReceiver {
    fn new(
        valid_responses: Receiver<ResponseInfo>,
        invalid_responses: Receiver<ResponseInfo>,
        errors: Receiver<reqwest::Error>,
    ) -> Self {
        return Self {
            valid_responses,
            invalid_responses,
            errors,
        };
    }

    pub fn valid_responses(&self) -> &Receiver<ResponseInfo> {
        return &self.valid_responses;
    }

    pub fn invalid_responses(&self) -> &Receiver<ResponseInfo> {
        return &self.invalid_responses;
    }

    pub fn errors(&self) -> &Receiver<reqwest::Error> {
        return &self.errors;
    }
}

pub struct EndCommunicator {
    receiver: Receiver<()>,
    sender: Sender<()>,
}

impl EndCommunicator {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<()>();
        return Self { sender, receiver };
    }

    pub fn sender(&self) -> &Sender<()> {
        return &self.sender;
    }

    pub fn receiver(&self) -> &Receiver<()> {
        return &self.receiver;
    }
}
