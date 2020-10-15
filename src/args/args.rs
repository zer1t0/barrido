use super::defs::args;
use crate::http::HttpOptions;
use crate::verificator::{
    BodyRegexVerificator, CodesVerificator, HeaderRegexVerificator,
    OrVerificator, SizeVerificator, Verificator,
};
use clap::ArgMatches;
use regex::Regex;
use reqwest::Proxy;
use std::collections::HashMap;
use std::time::Duration;

/// Class used to store the arguments provided by the user.
#[derive(Clone, Debug)]
pub struct Args {
    pub threads: usize,
    pub urls: Vec<String>,
    pub wordlist: String,
    pub out_file_json: Option<String>,
    pub proxy: Option<Proxy>,
    pub check_ssl: bool,
    pub expand_path: bool,
    pub codes_verification: CodesVerification,
    pub body_regex_verification: Option<BodyRegexVerification>,
    pub header_regex_verification: Option<HeaderRegexVerification>,
    pub size_range_verification: Option<RangeSizeVerification>,
    pub user_agent: String,
    pub show_status: bool,
    pub show_size: bool,
    pub show_progress: bool,
    pub show_headers: bool,
    pub path_suffix: String,
    pub use_scraper: bool,
    pub follow_redirects: bool,
    pub timeout: Duration,
    pub headers: HashMap<String, String>,
    pub verbosity: usize,
}

impl Args {
    pub fn parse_args() -> Self {
        let matches = args().get_matches();

        let threads: usize =
            matches.value_of("threads").unwrap().parse().unwrap();

        return Args {
            threads: threads,
            urls: urls(&matches),
            wordlist: wordlist(&matches),
            out_file_json: out_file_path(&matches),
            proxy: proxy(&matches),
            check_ssl: matches.is_present("check-ssl"),
            expand_path: matches.is_present("expand-path"),
            codes_verification: codes_verification(&matches),
            body_regex_verification: body_regex_verification(&matches),
            header_regex_verification: header_regex(&matches),
            size_range_verification: range_sizes_verification(&matches),
            user_agent: matches.value_of("user-agent").unwrap().to_string(),
            show_status: matches.is_present("status"),
            show_size: matches.is_present("size"),
            show_progress: matches.is_present("progress"),
            show_headers: matches.is_present("show-headers"),
            path_suffix: matches.value_of("suffix").unwrap_or("").to_string(),
            use_scraper: matches.is_present("scraper"),
            follow_redirects: matches.is_present("follow-redirects"),
            timeout: timeout(&matches),
            headers: headers(&matches),
            verbosity: matches.occurrences_of("verbosity") as usize,
        };
    }
}

fn headers(matches: &ArgMatches) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    if matches.is_present("header") {
        for header in matches.values_of("header").unwrap() {
            let mut parts = header.split(":");
            let header_name = parts.next().unwrap();
            let header_value = parts.collect::<Vec<&str>>().join(":");

            headers.insert(
                header_name.to_string(),
                header_value.trim().to_string(),
            );
        }
    }
    return headers;
}

fn codes_verification(matches: &ArgMatches) -> CodesVerification {
    let codes_verification: CodesVerification;
    if matches.is_present("filter-codes") {
        let mut code_list: Vec<u16> = Vec::new();

        for code in matches.values_of("filter-codes").unwrap() {
            code_list.push(
                code.parse()
                    .expect(format!("Invalid code {}", code).as_str()),
            )
        }

        codes_verification = CodesVerification::InvalidCodes(code_list);
    } else {
        let mut code_list: Vec<u16> = Vec::new();

        for code in matches.values_of("match-codes").unwrap() {
            code_list.push(
                code.parse()
                    .expect(format!("Invalid code {}", code).as_str()),
            )
        }

        codes_verification = CodesVerification::ValidCodes(code_list);
    }
    return codes_verification;
}

fn out_file_path(matches: &ArgMatches) -> Option<String> {
    return Some(matches.value_of("out-file")?.to_string());
}

fn proxy(matches: &ArgMatches) -> Option<Proxy> {
    return Some(Proxy::all(matches.value_of("proxy")?).unwrap());
}

fn range_sizes_verification(
    matches: &ArgMatches,
) -> Option<RangeSizeVerification> {
    if let Some(sizes) = parse_range_sizes(matches, "match-size") {
        return Some(RangeSizeVerification::MatchSize(sizes));
    }

    if let Some(sizes) = parse_range_sizes(matches, "filter-size") {
        return Some(RangeSizeVerification::FilterSize(sizes));
    }

    return None;
}

fn parse_range_sizes(
    matches: &ArgMatches,
    name: &str,
) -> Option<Vec<(usize, usize)>> {
    if let Some(size_ranges) = matches.values_of(name) {
        let mut ranges = Vec::new();
        for size_range in size_ranges {
            let parts: Vec<&str> = size_range.split("-").collect();

            let range = match parts.len() {
                1 => {
                    let size = parts[0].parse().unwrap();
                    (size, size)
                }
                2 => {
                    let min_size_str = parts[0];
                    let max_size_str = parts[1];

                    let min_size = match min_size_str {
                        "*" => 0,
                        _ => min_size_str.parse().unwrap(),
                    };

                    let max_size = match max_size_str {
                        "*" => usize::max_value(),
                        _ => max_size_str.parse().unwrap(),
                    };
                    (min_size, max_size)
                }
                _ => unreachable!(),
            };

            ranges.push(range);
        }
        return Some(ranges);
    }

    return None;
}

fn body_regex_verification(
    matches: &ArgMatches,
) -> Option<BodyRegexVerification> {
    if matches.is_present("filter-body") {
        return Some(BodyRegexVerification::InvalidRegex(
            Regex::new(matches.value_of("filter-body").unwrap())
                .expect("Error parsing filter-body"),
        ));
    } else if matches.is_present("match-body") {
        return Some(BodyRegexVerification::ValidRegex(
            Regex::new(matches.value_of("match-body").unwrap())
                .expect("Error parsing filter-body"),
        ));
    }
    return None;
}

fn timeout(matches: &ArgMatches) -> Duration {
    let timeout_secs: usize =
        matches.value_of("timeout").unwrap().parse().unwrap();

    return Duration::from_secs(timeout_secs as u64);
}

fn urls(matches: &ArgMatches) -> Vec<String> {
    match matches.values_of("url") {
        None => Vec::new(),
        Some(urls) => urls.map(|u| u.to_string()).collect(),
    }
}

fn header_regex(matches: &ArgMatches) -> Option<HeaderRegexVerification> {
    if let Some(value) = matches.value_of("match-header") {
        let (name_regex, value_regex) = gen_header_regex(value);
        return Some(HeaderRegexVerification::ValidRegex(
            name_regex,
            value_regex,
        ));
    } else if let Some(value) = matches.value_of("filter-header") {
        let (name_regex, value_regex) = gen_header_regex(value);
        return Some(HeaderRegexVerification::InvalidRegex(
            name_regex,
            value_regex,
        ));
    }

    return None;
}

fn gen_header_regex(value: &str) -> (Regex, Regex) {
    let mut parts: Vec<&str> = value.split(":").collect();

    if parts.len() == 1 {
        return (new_insensitive_regex(parts[0]), Regex::new(".*").unwrap());
    }

    let name = parts.remove(0);
    let name_regex = if name == "" {
        Regex::new(".*").unwrap()
    } else {
        new_insensitive_regex(name)
    };

    let value = parts.join(":");
    let value_regex = if value == "" {
        Regex::new(".*").unwrap()
    } else {
        Regex::new(&value).unwrap()
    };

    return (name_regex, value_regex);
}

fn wordlist(matches: &ArgMatches) -> String {
    match matches.value_of("wordlist") {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}

fn new_insensitive_regex(v: &str) -> Regex {
    Regex::new(&format!("(?i){}", v)).unwrap()
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum BodyRegexVerification {
    ValidRegex(Regex),
    InvalidRegex(Regex),
}

impl Into<Verificator> for BodyRegexVerification {
    fn into(self) -> Verificator {
        match self {
            BodyRegexVerification::ValidRegex(re) => {
                BodyRegexVerificator::new(re)
            }
            BodyRegexVerification::InvalidRegex(re) => {
                !BodyRegexVerificator::new(re)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum HeaderRegexVerification {
    ValidRegex(Regex, Regex),
    InvalidRegex(Regex, Regex),
}

impl Into<Verificator> for HeaderRegexVerification {
    fn into(self) -> Verificator {
        match self {
            HeaderRegexVerification::ValidRegex(re1, re2) => {
                HeaderRegexVerificator::new(re1, re2)
            }
            HeaderRegexVerification::InvalidRegex(re1, re2) => {
                !HeaderRegexVerificator::new(re1, re2)
            }
        }
    }
}
