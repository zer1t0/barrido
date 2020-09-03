mod arguments;
pub mod discoverer;
mod printer;
mod result_handler;
mod result_saver;

use ctrlc;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crossbeam_channel;

use discoverer::http::HttpOptions;
use discoverer::verificator::{
    CodesVerificator, OrVerificator, RegexVerificator, SizeVerificator,
    TrueVerificator, Verificator,
};
use discoverer::DiscovererBuilder;

use printer::Printer;
use reqwest::Url;
use result_handler::ResultHandler;
use result_saver::JsonResultSaver;

use arguments::{Arguments, CodesVerification, RangeSizeVerification};

fn main() {
    env_logger::init();
    let args = Arguments::parse_args();

    let wordlist =
        File::open(&args.wordlist).expect("Error opening wordlist file");

    let paths_reader = BufReader::new(wordlist);
    let paths: Vec<String> = paths_reader
        .lines()
        .map(|l| l.expect("error parsing line"))
        .collect();

    let http_options: HttpOptions = args.clone().into();

    let verificator = generate_verificator(&args);

    let base_urls = parse_urls(&args.urls);

    let max_requests_count = paths.len() * base_urls.len();

    let discoverer = DiscovererBuilder::new(base_urls, paths)
        .requesters_count(args.threads)
        .verificator(verificator)
        .http_options(http_options)
        .use_scraper(args.use_scraper)
        .spawn();

    let (signal_sender, signal_receiver) = crossbeam_channel::unbounded::<()>();

    spawn_signal_handler(signal_sender);

    let printer = Printer::new(
        args.verbosity,
        args.show_status,
        args.show_size,
        args.show_progress,
        args.expand_path,
    );

    let results = ResultHandler::start(
        discoverer.result_receiver().clone(),
        discoverer.end_receiver().clone(),
        signal_receiver,
        max_requests_count,
        printer,
    );

    if let Some(out_file_path) = &args.out_file_json {
        JsonResultSaver::save_results(&results, out_file_path);
    }
}

fn generate_verificator(args: &Arguments) -> Verificator {
    let codes_verificator = generate_codes_verificator(args);
    let regex_verificator = generate_regex_verificator(args);
    let sizes_verificator = generate_sizes_verificator(args);

    return codes_verificator & regex_verificator & sizes_verificator;
}

fn generate_codes_verificator(args: &Arguments) -> Verificator {
    match &args.codes_verification {
        CodesVerification::ValidCodes(codes) => {
            CodesVerificator::new(codes.clone())
        }
        CodesVerification::InvalidCodes(codes) => {
            !CodesVerificator::new(codes.clone())
        }
    }
}

fn generate_regex_verificator(args: &Arguments) -> Verificator {
    match &args.regex_verification {
        Some(filter_regex) => !RegexVerificator::new(filter_regex.clone()),
        None => TrueVerificator::new(),
    }
}

fn generate_sizes_verificator(args: &Arguments) -> Verificator {
    let range_verificator = match &args.size_range_verification {
        Some(size_range_verification) => match size_range_verification {
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
        },
        None => TrueVerificator::new(),
    };

    return range_verificator;
}

fn spawn_signal_handler(sender: crossbeam_channel::Sender<()>) {
    ctrlc::set_handler(move || {
        sender
            .send(())
            .expect("SignalHandler: error sending signal");
    })
    .unwrap();
}

fn parse_urls(urls: &str) -> Vec<Url> {
    let mut base_urls = Vec::new();
    match File::open(urls) {
        Ok(urls_file) => {
            let file_reader = BufReader::new(urls_file);
            for line in file_reader.lines() {
                let url_str = line.unwrap();
                match Url::parse(&url_str) {
                    Ok(url) => base_urls.push(url),
                    Err(_) => {
                        println!("[X] {} is not a valid URL", url_str);
                        std::process::exit(-1);
                    }
                }
            }
        }
        Err(_) => {
            let mut url_str = urls.to_string();

            if !url_str.ends_with("/") {
                url_str.push('/');
            }
            if let Ok(base_url) = Url::parse(&url_str) {
                base_urls.push(base_url);
            } else {
                println!("[X] {} is not a valid URL", url_str);
                std::process::exit(-1);
            }
        }
    };
    return base_urls;
}
