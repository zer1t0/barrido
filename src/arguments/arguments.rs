use super::defs::args;
use super::parser::ArgumentsParser;
use crate::http::HttpOptions;
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
#[derive(Clone)]
pub struct Arguments {
    pub threads: usize,
    pub urls: String,
    pub wordlist: String,
    pub out_file_json: Option<String>,
    pub proxy: Option<Proxy>,
    pub check_ssl: bool,
    pub expand_path: bool,
    pub codes_verification: CodesVerification,
    pub regex_verification: Option<Regex>,
    pub size_range_verification: Option<RangeSizeVerification>,
    pub user_agent: String,
    pub show_status: bool,
    pub show_size: bool,
    pub show_progress: bool,
    pub show_headers: bool,
    pub use_scraper: bool,
    pub follow_redirects: bool,
    pub timeout: Duration,
    pub headers: HashMap<String, String>,
    pub verbosity: usize,
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
