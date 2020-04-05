use crate::discoverer::communication::Receiver;
use crate::discoverer::http::Url;

pub trait UrlProvider {
    fn receiver(&self) -> &Receiver<Url>;
}
