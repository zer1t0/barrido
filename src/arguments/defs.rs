use clap::{App, Arg};
use reqwest::Proxy;

pub fn args() -> App<'static, 'static> {
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
            Arg::with_name("size")
                .long("size")
                .short("l")
                .help("Show the size of the response"),
        )
        .arg(
            Arg::with_name("progress")
                .long("progress")
                .short("p")
                .help("Show the progress of requests"),
        )
        .arg(
            Arg::with_name("show-headers")
                .long("head")
                .short("I")
                .help("Show the reponse headers")
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
            Arg::with_name("match-size")
                .long("match-size")
                .help("Exact length of responses (e.g. 94,100-200,300-*,*-600)")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true)
                .validator(is_usize_or_range),
        )
        .arg(
            Arg::with_name("filter-size")
                .long("filter-size")
                .help("Exact size of invalid responses (e.g. 94,100-200,300-*,*-600)")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true)
                .validator(is_usize_or_range)
                .conflicts_with_all(&["match-size"]),
        )
}

fn is_proxy(v: String) -> Result<(), String> {
    match Proxy::all(&v) {
        Ok(_) => Ok(()),
        Err(_) => Err("Must be an URL".to_string()),
    }
}

fn is_usize_or_range(v: String) -> Result<(), String> {
    let parts: Vec<&str> = v.split("-").collect();

    if parts.len() == 1 {
        return is_usize(v);
    }

    if parts.len() != 2 {
        return Err("Range must be two parts separated by '-'".to_string());
    }
    let min_size = parts[0];
    let max_size = parts[1];

    if min_size != "*" {
        match min_size.parse::<usize>() {
            Err(_) => {
                return Err("Range parts must be numbers or *".to_string());
            }
            Ok(_) => {}
        }
    }

    if max_size != "*" {
        match max_size.parse::<usize>() {
            Err(_) => {
                return Err("Range parts must be numbers or *".to_string());
            }
            Ok(_) => {}
        }
    }

    return Ok(());
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
