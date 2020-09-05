use crate::communication::Receiver;
use crate::http::Url;

pub trait UrlProvider {
    fn receiver(&self) -> &Receiver<Url>;
}
