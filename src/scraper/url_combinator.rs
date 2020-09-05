use crate::http::Url;
use log::warn;


pub struct UrlCombinator<T>
where
    T: Iterator<Item = String>,
{
    url: Url,
    paths: T,
    created_urls: Vec<Url>,
}

impl<T> UrlCombinator<T>
where
    T: Iterator<Item = String>,
{
    pub fn new(url: Url, paths: T) -> Self {
        return Self {
            url,
            paths,
            created_urls: Vec::new(),
        };
    }
}

impl<T> Iterator for UrlCombinator<T>
where
    T: Iterator<Item = String>,
{
    type Item = Url;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let path = self.paths.next()?;

            match self.url.join(&path) {
                Ok(mut url) => {
                    url.set_fragment(None);
                    url.set_query(None);

                    if self.created_urls.contains(&url) {
                        continue;
                    }
                    self.created_urls.push(url.clone());

                    return Some(url);
                }
                Err(error) => {
                    warn!(
                        "Error joining url error({}) url({}) path({})",
                        error, self.url, path
                    );
                }
            }
        }
    }
}
