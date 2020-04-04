use super::defs::args;
use super::parser::ArgumentsParser;
use crate::discoverer::http::HttpOptions;
use derive_builder::Builder;
use getset::Getters;
use regex::Regex;
use reqwest::Proxy;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone)]
pub enum CodesVerification {
    ValidCodes(Vec<u16>),
    InvalidCodes(Vec<u16>),
}

#[derive(Clone)]
pub enum RangeSizeVerification {
    MatchSize(Vec<(usize, usize)>),
    FilterSize(Vec<(usize, usize)>),
}

/// Class used to store the arguments provided by the user.
#[derive(Clone, Getters, Builder)]
#[getset(get = "pub")]
pub struct Arguments {
    threads: usize,
    urls: String,
    wordlist: String,
    out_file_json: Option<String>,
    proxy: Option<Proxy>,
    check_ssl: bool,
    expand_path: bool,
    codes_verification: CodesVerification,
    regex_verification: Option<Regex>,
    size_range_verification: Option<RangeSizeVerification>,
    user_agent: String,
    show_status: bool,
    show_size: bool,
    show_progress: bool,
    use_scraper: bool,
    follow_redirects: bool,
    timeout: Duration,
    headers: HashMap<String, String>,
    verbosity: u64,
}

impl Arguments {
    pub fn parse_args() -> Self {
        let arg_matches = args().get_matches();
        return ArgumentsParser::new(arg_matches).parse_args();
    }
}

impl Into<HttpOptions> for Arguments {
    fn into(self) -> HttpOptions {
        return HttpOptions::new(
            self.check_ssl,
            self.follow_redirects,
            self.proxy,
            self.user_agent,
            self.timeout,
            self.headers,
        );
    }
}
