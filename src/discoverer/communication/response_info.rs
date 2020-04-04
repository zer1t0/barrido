use crate::discoverer::http::Response;
use reqwest::Url;

#[derive(Debug)]
pub struct ResponseInfo {
    response: Response,
}

impl ResponseInfo {
    pub fn new(response: Response) -> Self {
        return Self { response };
    }

    pub fn status(&self) -> u16 {
        return self.response.status();
    }

    pub fn url(&self) -> &Url {
        return self.response.url();
    }

    pub fn body_length(&self) -> usize {
        return self.response.body().len();
    }
}
