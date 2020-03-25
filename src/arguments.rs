use super::discoverer::http_client::HttpOptions;
use clap::*;
use regex::Regex;
use reqwest::Proxy;
use std::collections::HashMap;
use std::result::Result;
use std::time::Duration;

fn args() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("url")
                .takes_value(true)
                .help("url to load")
                .required(true),
        )
        .arg(
            Arg::with_name("wordlist")
                .takes_value(true)
                .help("list of paths")
                .required(true),
        )
        .arg(
            Arg::with_name("threads")
                .long("threads")
                .short("t")
                .takes_value(true)
                .help("Number of threads")
                .default_value("10")
                .validator(is_usize_major_than_zero),
        )
        .arg(
            Arg::with_name("out-file")
                .long("out-file")
                .short("-o")
                .takes_value(true)
                .help("File to write results (json format)"),
        )
        .arg(
            Arg::with_name("proxy")
                .long("proxy")
                .short("x")
                .takes_value(true)
                .validator(is_proxy)
                .help("Specify proxy in format: http[s]://<host>[:<port>]"),
        )
        .arg(
            Arg::with_name("insecure")
                .long("insecure")
                .short("k")
                .help("Allow insecure connections when using SSL"),
        )
        .arg(
            Arg::with_name("valid-codes")
                .long("valid-codes")
                .help("Response codes which are valid")
                .takes_value(true)
                .use_delimiter(true)
                .default_value("200,204,301,302,307,401,403"),
        )
        .arg(
            Arg::with_name("invalid-codes")
                .long("invalid-codes")
                .help("Response codes which are invalid")
                .takes_value(true)
                .use_delimiter(true)
                .conflicts_with("valid-codes"),
        )
        .arg(
            Arg::with_name("invalid-regex")
                .long("invalid-regex")
                .help("Regex to match invalid responses")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("user-agent")
                .long("user-agent")
                .short("A")
                .help("Set custom User-Agent")
                .takes_value(true)
                .default_value("barrido"),
        )
        .arg(
            Arg::with_name("expand-path")
                .long("expand-path")
                .short("e")
                .help("Return paths with the complete url"),
        )
        .arg(
            Arg::with_name("status")
                .long("status")
                .short("s")
                .help("Show the discovered paths with the response code"),
        )
        .arg(
            Arg::with_name("body-length")
                .long("body-length")
                .short("l")
                .help("Show the discovered paths with the response code"),
        )
        .arg(
            Arg::with_name("progress")
                .long("progress")
                .short("p")
                .help("Show the progress of requests"),
        )
        .arg(
            Arg::with_name("scraper")
                .long("scraper")
                .help("Scrap for new paths in responses"),
        )
        .arg(
            Arg::with_name("follow-redirects")
                .long("follow-redirects")
                .alias("follow-redirect")
                .help("Follow HTTP redirections"),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .help("HTTP requests timeout")
                .takes_value(true)
                .default_value("10")
                .validator(is_usize_major_than_zero),
        )
        .arg(
            Arg::with_name("header")
                .long("header")
                .short("H")
                .help("Headers to send in request")
                .takes_value(true)
                .multiple(true)
                .validator(is_header),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Verbosity"),
        )
        .arg(
            Arg::with_name("max-length")
                .long("max-length")
                .help("Maximum length in responses")
                .takes_value(true)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("min-length")
                .long("min-length")
                .help("Minimum length in responses")
                .takes_value(true)
                .validator(is_usize),
        )
        .arg(
            Arg::with_name("length")
                .long("exact-length")
                .help("Exact length of responses")
                .takes_value(true)
                .validator(is_usize)
                .conflicts_with_all(&["min-length", "max-length"]),
        )
        .arg(
            Arg::with_name("no-length")
                .long("no-exact-length")
                .help("Exact length of invalid responses")
                .takes_value(true)
                .validator(is_usize)
                .conflicts_with_all(&["length", "min-length", "max-length"]),
        )
}

fn is_proxy(v: String) -> Result<(), String> {
    match Proxy::all(&v) {
        Ok(_) => Ok(()),
        Err(_) => Err("Must be an URL".to_string()),
    }
}

fn is_usize(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Must be a positive integer bigger than 0".to_string()),
    }
}

fn is_usize_major_than_zero(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(uint) => {
            if uint == 0 {
                return Err(
                    "Must be a positive integer bigger than 0".to_string()
                );
            }
            Ok(())
        }
        Err(_) => Err("Must be a positive integer bigger than 0".to_string()),
    }
}

fn is_header(v: String) -> Result<(), String> {
    let parts = v.split(":");

    if parts.collect::<Vec<&str>>().len() < 2 {
        return Err(format!("\"{}\" is not in the format `Name: Value`", v));
    }
    return Ok(());
}

#[derive(Clone)]
pub enum CodesVerification {
    ValidCodes(Vec<u16>),
    InvalidCodes(Vec<u16>),
}

#[derive(Clone)]
pub enum SizeVerification {
    ExactValidSize(usize),
    ExactInvalidSize(usize),
    RangeSize(usize, usize),
}

#[derive(Clone)]
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
    size_verification: Option<SizeVerification>,
    user_agent: String,
    show_status: bool,
    show_body_length: bool,
    show_progress: bool,
    use_scraper: bool,
    follow_redirects: bool,
    timeout: Duration,
    headers: HashMap<String, String>,
    verbosity: u64,
}

impl Arguments {
    pub fn threads(&self) -> usize {
        return self.threads;
    }

    pub fn urls(&self) -> &String {
        return &self.urls;
    }

    pub fn wordlist(&self) -> &String {
        return &self.wordlist;
    }

    pub fn out_file_json(&self) -> Option<&String> {
        return self.out_file_json.as_ref();
    }

    pub fn expand_path(&self) -> bool {
        return self.expand_path;
    }

    pub fn codes_verification(&self) -> &CodesVerification {
        return &self.codes_verification;
    }

    pub fn regex_verification(&self) -> Option<&Regex> {
        return self.regex_verification.as_ref();
    }

    pub fn size_verification(&self) -> Option<&SizeVerification> {
        return self.size_verification.as_ref();
    }

    pub fn show_status(&self) -> bool {
        return self.show_status;
    }

    pub fn show_progress(&self) -> bool {
        return self.show_progress;
    }

    pub fn show_body_length(&self) -> bool {
        return self.show_body_length;
    }

    pub fn use_scraper(&self) -> bool {
        return self.use_scraper;
    }

    pub fn verbosity(&self) -> u64 {
        return self.verbosity;
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

pub fn parse_args() -> Arguments {
    let matches = args().get_matches();

    return ArgumentsBuilder::new(matches).parse_args();
}

struct ArgumentsBuilder<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> ArgumentsBuilder<'a> {
    fn new(matches: ArgMatches<'a>) -> Self {
        return Self { matches };
    }

    fn parse_args(&self) -> Arguments {
        let out_file_path = self.out_file_path();

        let urls = self.value_of("url").unwrap().to_string();

        let wordlist_path = self.value_of("wordlist").unwrap();
        let threads: usize = self.value_of("threads").unwrap().parse().unwrap();

        let timeout = self.timeout();

        let proxy = self.proxy();

        let codes_verification = self.codes_verification();

        let regex_verification = self.regex_verification();

        let headers = self.headers();

        return Arguments {
            threads,
            urls,
            wordlist: wordlist_path.to_string(),
            out_file_json: out_file_path,
            proxy,
            check_ssl: !self.is_present("insecure"),
            expand_path: self.is_present("expand-path"),
            codes_verification,
            regex_verification,
            size_verification: self.size_verification(),
            user_agent: self.value_of("user-agent").unwrap().to_string(),
            show_status: self.is_present("status"),
            show_progress: self.is_present("progress"),
            show_body_length: self.is_present("body-length"),
            use_scraper: self.is_present("scraper"),
            follow_redirects: self.is_present("follow-redirects"),
            timeout,
            headers,
            verbosity: self.matches.occurrences_of("verbosity"),
        };
    }

    fn out_file_path(&self) -> Option<String> {
        return Some(self.value_of("out-file")?.to_string());
    }

    fn proxy(&self) -> Option<Proxy> {
        return Some(Proxy::all(self.value_of("proxy")?).unwrap());
    }

    fn timeout(&self) -> Duration {
        let timeout_secs: usize =
            self.value_of("timeout").unwrap().parse().unwrap();

        return Duration::from_secs(timeout_secs as u64);
    }

    fn codes_verification(&self) -> CodesVerification {
        let codes_verification: CodesVerification;
        if self.is_present("invalid-codes") {
            let mut code_list: Vec<u16> = Vec::new();

            for code in self.values_of("invalid-codes").unwrap() {
                code_list.push(
                    code.parse()
                        .expect(format!("Invalid code {}", code).as_str()),
                )
            }

            codes_verification = CodesVerification::InvalidCodes(code_list);
        } else {
            let mut code_list: Vec<u16> = Vec::new();

            for code in self.values_of("valid-codes").unwrap() {
                code_list.push(
                    code.parse()
                        .expect(format!("Invalid code {}", code).as_str()),
                )
            }

            codes_verification = CodesVerification::ValidCodes(code_list);
        }
        return codes_verification;
    }

    fn regex_verification(&self) -> Option<Regex> {
        if self.is_present("invalid-regex") {
            return Some(
                Regex::new(self.value_of("invalid-regex").unwrap())
                    .expect("Error parsing invalid-regex"),
            );
        }
        return None;
    }

    fn headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        if self.is_present("header") {
            for header in self.values_of("header").unwrap() {
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

    fn size_verification(&self) -> Option<SizeVerification> {
        if self.is_present("no-length") {
            let length = self.value_of("no-length").unwrap().parse().unwrap();
            return Some(SizeVerification::ExactInvalidSize(length));
        } else if self.is_present("length") {
            let length = self.value_of("length").unwrap().parse().unwrap();
            return Some(SizeVerification::ExactValidSize(length));
        } else if self.is_present("min-length") || self.is_present("max-length")
        {
            let min_length;
            let max_length;

            if self.is_present("min-length") {
                min_length =
                    self.value_of("min-length").unwrap().parse().unwrap();
            } else {
                min_length = 0;
            }

            if self.is_present("max-length") {
                max_length =
                    self.value_of("max-length").unwrap().parse().unwrap();
            } else {
                max_length = usize::max_value();
            }
            return Some(SizeVerification::RangeSize(min_length, max_length));
        }

        return None;
    }

    fn value_of(&self, k: &str) -> Option<&str> {
        return self.matches.value_of(k);
    }

    fn values_of(&self, k: &str) -> Option<Values<'_>> {
        return self.matches.values_of(k);
    }

    fn is_present(&self, k: &str) -> bool {
        return self.matches.is_present(k);
    }
}
