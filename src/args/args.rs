use super::defs::args;
use super::parser::ArgumentsParser;
use crate::http::HttpOptions;
use crate::verificator::{
    CodesVerificator, OrVerificator, SizeVerificator, Verificator,
};
use regex::Regex;
use reqwest::Proxy;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone)]
pub enum CodesVerification {
    ValidCodes(Vec<u16>),
    InvalidCodes(Vec<u16>),
}

impl Into<Verificator> for CodesVerification {
    fn into(self) -> Verificator {
        match self {
            CodesVerification::ValidCodes(codes) => {
                CodesVerificator::new(codes)
            }
            CodesVerification::InvalidCodes(codes) => {
                !CodesVerificator::new(codes)
            }
        }
    }
}

#[derive(Clone)]
pub enum RangeSizeVerification {
    MatchSize(Vec<(usize, usize)>),
    FilterSize(Vec<(usize, usize)>),
}

impl Into<Verificator> for RangeSizeVerification {
    fn into(self) -> Verificator {
        match self {
            RangeSizeVerification::MatchSize(ranges) => OrVerificator::new(
                ranges
                    .iter()
                    .map(|r| SizeVerificator::new_range(r.0, r.1))
                    .collect(),
            ),
            RangeSizeVerification::FilterSize(ranges) => !OrVerificator::new(
                ranges
                    .iter()
                    .map(|r| SizeVerificator::new_range(r.0, r.1))
                    .collect(),
            ),
        }
    }
}

/// Class used to store the arguments provided by the user.
#[derive(Clone)]
pub struct Args {
    pub threads: usize,
    pub urls: String,
    pub wordlist: String,
    pub out_file_json: Option<String>,
    pub proxy: Option<Proxy>,
    pub check_ssl: bool,
    pub expand_path: bool,
    pub codes_verification: CodesVerification,
    pub regex_verification: Option<Regex>,
    pub valid_header_regex_verification: Option<(Regex, Regex)>,
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

impl Args {
    pub fn parse_args() -> Self {
        let arg_matches = args().get_matches();
        return ArgumentsParser::new(arg_matches).parse_args();
    }
}

impl Into<HttpOptions> for Args {
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
