use super::arguments::{Arguments, ArgumentsBuilder,CodesVerification, RangeSizeVerification};
use clap::{ArgMatches, Values};
use regex::Regex;
use reqwest::Proxy;
use std::collections::HashMap;
use std::time::Duration;

/// To parse the arguments from the clap representation to the Argument struct representation
pub struct ArgumentsParser<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> ArgumentsParser<'a> {
    pub fn new(matches: ArgMatches<'a>) -> Self {
        return Self { matches };
    }

    pub fn parse_args(&self) -> Arguments {
        let wordlist_path = self.value_of("wordlist").unwrap();
        let threads: usize = self.value_of("threads").unwrap().parse().unwrap();

        return ArgumentsBuilder::default()
            .check_ssl(!self.is_present("insecure"))
            .codes_verification(self.codes_verification())
            .expand_path(self.is_present("expand-path"))
            .follow_redirects(self.is_present("follow-redirects"))
            .headers(self.headers())
            .out_file_json(self.out_file_path())
            .proxy(self.proxy())
            .regex_verification(self.regex_verification())
            .size_range_verification(self.parse_range_sizes_verification())
            .show_status(self.is_present("status"))
            .show_progress(self.is_present("progress"))
            .show_size(self.is_present("size"))
            .threads(threads)
            .timeout(self.timeout())
            .urls(self.value_of("url").unwrap().to_string())
            .use_scraper(self.is_present("scraper"))
            .user_agent(self.value_of("user-agent").unwrap().to_string())
            .verbosity(self.matches.occurrences_of("verbosity") as usize)
            .wordlist(wordlist_path.to_string())
            .build().expect("Error building arguments");

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

    fn parse_range_sizes_verification(&self) -> Option<RangeSizeVerification> {
        if let Some(sizes) = self.parse_range_sizes("match-size") {
            return Some(RangeSizeVerification::MatchSize(sizes));
        }

        if let Some(sizes) = self.parse_range_sizes("filter-size") {
            return Some(RangeSizeVerification::FilterSize(sizes));
        }

        return None;
    }

    fn parse_range_sizes(&self, name: &str) -> Option<Vec<(usize, usize)>> {
        if let Some(size_ranges) = self.values_of(name) {
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
