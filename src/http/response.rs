use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Url;

#[derive(Debug)]
pub struct Response {
    response: reqwest::Response,
    body: String,
}

impl Response {
    pub fn body(&self) -> &String {
        return &self.body;
    }

    pub fn status(&self) -> u16 {
        return self.response.status().as_u16();
    }

    pub fn url(&self) -> &Url {
        return self.response.url();
    }

    pub fn headers(&self) -> &HeaderMap {
        self.response.headers()
    }

    pub fn content_type(&self) -> Option<ContentType> {
        let headers = self.response.headers();

        let content_type = headers.get(CONTENT_TYPE)?;
        let content_type =
            content_type.to_str().expect("Unable to parse Content-Type");

        let mut name = "";
        for part in content_type.split(";") {
            name = part;
            break;
        }

        return ContentType::from_str(name);
    }
}

impl From<reqwest::Response> for Response {
    fn from(mut response: reqwest::Response) -> Self {
        return Self {
            body: response.text().expect("Error parsing response body"),
            response,
        };
    }
}

pub enum ContentType {
    Html,
    Javascript,
}

impl ContentType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "text/html" | "application/xhtml+xml" => Some(ContentType::Html),
            "text/javascript"
            | "application/javascript"
            | "application/x-javascript" => Some(ContentType::Javascript),
            _ => None,
        }
    }
}
