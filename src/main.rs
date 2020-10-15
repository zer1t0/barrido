mod actors;
mod args;
mod communication;
mod http;
mod printer;
mod readin;
mod result_handler;
mod result_saver;
mod scraper;
mod verificator;

use ctrlc;

use crossbeam_channel;

use crate::http::HttpOptions;
use crate::verificator::{Verificator};

use std::collections::HashSet;

use printer::Printer;
use reqwest::Url;
use result_handler::ResultHandler;
use result_saver::JsonResultSaver;

use args::{
    Args, BodyRegexVerification, CodesVerification, HeaderRegexVerification,
    RangeSizeVerification,
};

use crossbeam_channel::Receiver;
use std::sync::Arc;
use std::thread;
use threadpool::ThreadPool;

use crate::actors::{EndChecker, Requester, ResponseHandler, UrlAggregator};
use crate::communication::result_channel::{
    ResultChannel, ResultReceiver, ResultSender,
};
use crate::communication::{
    new_wait_mutex, new_wait_mutex_vec, EndChannel, ResponseChannel,
    ResponseReceiver, ResponseSender, UrlChannel, UrlReceiver, UrlSender,
    UrlsReceiver, WaitMutex,
};
use crate::scraper::{
    EmptyScraperProvider, ScraperProvider, UrlsScraperProvider,
};
use log::{debug, info, warn};
use stderrlog;

fn main() {
    let args = Args::parse_args();
    init_log(args.verbosity);
    debug!("{:?}", args);

    let http_options: HttpOptions = args.clone().into();
    let verificator = gen_verificator(
        args.codes_verification,
        args.body_regex_verification,
        args.header_regex_verification,
        args.size_range_verification,
    );

    let paths: Vec<String> = read_paths(vec![args.wordlist], &args.path_suffix);
    let base_urls = read_urls(args.urls);

    let max_requests_count = paths.len() * base_urls.len();

    let url_client = http_options.into();

    let discoverer = spawn_actors(
        args.use_scraper,
        args.threads,
        verificator,
        url_client,
        base_urls,
        paths,
    );

    let (signal_sender, signal_receiver) = crossbeam_channel::unbounded::<()>();

    spawn_signal_handler(signal_sender);

    let printer = Printer::new(
        args.show_status,
        args.show_size,
        args.show_progress,
        args.expand_path,
        args.show_headers,
    );

    let results = ResultHandler::start(
        discoverer.result_channel_receiver,
        discoverer.end_channel_receiver,
        signal_receiver,
        max_requests_count,
        printer,
    );

    if let Some(out_file_path) = &args.out_file_json {
        info!("Saving json results in {}", out_file_path);
        JsonResultSaver::save_results(&results, out_file_path);
    }
}

fn init_log(verbosity: usize) {
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity)
        .init()
        .expect("Error initiating log");
}

/// Function to read the paths or file of paths given.
/// It returns a vector of non duplicate paths.
fn read_paths(paths: Vec<String>, suffix: &str) -> Vec<String> {
    // to keep paths order
    let mut resolved_paths = Vec::new();

    // to increase check speed and remove duplicates in large wordlists
    let mut paths_set = HashSet::new();
    for mut path in readin::read_inputs(paths, false, false) {
        path = format!("{}{}", path, suffix);
        if !paths_set.contains(&path) {
            resolved_paths.push(path.clone());
            paths_set.insert(path);
        }
    }
    return resolved_paths;
}

/// Read urls from string, file or stdin if nothing else
/// is specified.
fn read_urls(urls: Vec<String>) -> Vec<Url> {
    let mut base_urls = Vec::new();

    for url_str in readin::read_inputs(urls, true, true) {
        match Url::parse(&url_str) {
            Ok(url) => {
                if !base_urls.contains(&url) {
                    base_urls.push(url);
                }
            }
            Err(_) => {
                warn!("Invalid URL: '{}'", url_str);
            }
        }
    }
    return base_urls;
}

fn gen_verificator(
    codes_verification: CodesVerification,
    body_regex_verification: Option<BodyRegexVerification>,
    header_regex_verification: Option<HeaderRegexVerification>,
    range_size_verification: Option<RangeSizeVerification>,
) -> Verificator {
    let codes_verificator: Verificator = codes_verification.into();
    let body_verificator = body_regex_verification.map(|brv| brv.into());
    let header_verificator = header_regex_verification.map(|hr| hr.into());
    let sizes_verificator: Option<Verificator> =
        range_size_verification.map(|rs| rs.into());

    return codes_verificator
        & body_verificator
        & header_verificator
        & sizes_verificator;
}

fn spawn_signal_handler(sender: crossbeam_channel::Sender<()>) {
    ctrlc::set_handler(move || {
        sender
            .send(())
            .expect("SignalHandler: error sending signal");
    })
    .unwrap();
}

fn spawn_actors(
    use_scraper: bool,
    requesters_count: usize,
    response_verificator: Verificator,
    client: reqwest::Client,
    base_urls: Vec<Url>,
    paths: Vec<String>,
) -> Discoverer {
    let response_handlers_count = 1;
    let scraper = create_scraper(use_scraper);
    let response_handlers_wait_mutexes =
        new_wait_mutex_vec(response_handlers_count);

    let requesters_wait_mutexes = new_wait_mutex_vec(requesters_count);

    let paths_provider_wait_mutex = new_wait_mutex();

    let response_channel = ResponseChannel::default();
    let result_channel = ResultChannel::default();

    let end_channel = EndChannel::default();
    let url_channel = UrlChannel::with_capacity(requesters_count * 4);

    let urls_receiver = scraper.receiver().clone();

    let response_handlers_pool = spawn_response_handlers(
        &response_handlers_wait_mutexes,
        scraper,
        response_channel.receiver,
        result_channel.sender,
        response_verificator,
    );
    let requesters_pool = spawn_requesters(
        &requesters_wait_mutexes,
        client,
        url_channel.receiver,
        response_channel.sender,
    );
    let paths_provider_pool = spawn_paths_provider(
        paths_provider_wait_mutex.clone(),
        base_urls,
        paths,
        url_channel.sender,
        urls_receiver,
    );

    spawn_end_checker(
        requesters_wait_mutexes,
        requesters_pool,
        response_handlers_wait_mutexes,
        response_handlers_pool,
        paths_provider_wait_mutex,
        paths_provider_pool,
        end_channel.sender,
    );

    return Discoverer::new(result_channel.receiver, end_channel.receiver);
}

fn create_scraper(use_scraper: bool) -> Box<dyn ScraperProvider> {
    match use_scraper {
        true => return Box::new(UrlsScraperProvider::new()),
        false => return Box::new(EmptyScraperProvider::new()),
    }
}

fn spawn_response_handlers(
    wait_mutexes: &Vec<WaitMutex>,
    scraper: Box<dyn ScraperProvider>,
    response_receiver: ResponseReceiver,
    result_sender: ResultSender,
    response_verificator: Verificator,
) -> ThreadPool {
    let response_handlers_pool =
        ThreadPool::with_name("Responsers".to_string(), wait_mutexes.len());
    let response_verificator = Arc::new(response_verificator);
    let scraper_arc = Arc::new(scraper);
    for (i, wait_mutex) in wait_mutexes.iter().enumerate() {
        let response_handler = ResponseHandler::new(
            response_receiver.clone(),
            result_sender.clone(),
            response_verificator.clone(),
            scraper_arc.clone(),
            wait_mutex.clone(),
            i,
        );
        response_handlers_pool.execute(move || {
            response_handler.run();
        });
    }

    return response_handlers_pool;
}

fn spawn_requesters(
    wait_mutexes: &Vec<WaitMutex>,
    client: reqwest::Client,
    url_receiver: UrlReceiver,
    response_sender: ResponseSender,
) -> ThreadPool {
    let requesters_pool =
        ThreadPool::with_name("Requesters".to_string(), wait_mutexes.len());

    let client = Arc::new(client);

    for (i, wait_mutex) in wait_mutexes.iter().enumerate() {
        let requester = Requester::new(
            client.clone(),
            url_receiver.clone(),
            response_sender.clone(),
            wait_mutex.clone(),
            i,
        );

        requesters_pool.execute(move || {
            requester.run();
        });
    }

    return requesters_pool;
}

fn spawn_paths_provider(
    wait_mutex: WaitMutex,
    base_urls: Vec<Url>,
    paths: Vec<String>,
    url_sender: UrlSender,
    scraper_receiver: UrlsReceiver,
) -> ThreadPool {
    let paths_provider_pool = ThreadPool::with_name("Providers".to_string(), 1);

    paths_provider_pool.execute(move || {
        UrlAggregator::new(url_sender, scraper_receiver, wait_mutex)
            .run(base_urls, paths);
    });

    return paths_provider_pool;
}

fn spawn_end_checker(
    requesters_wait_mutexes: Vec<WaitMutex>,
    requesters_pool: ThreadPool,
    response_handlers_wait_mutexes: Vec<WaitMutex>,
    response_handlers_pool: ThreadPool,
    paths_provider_wait_mutex: WaitMutex,
    paths_provider_pool: ThreadPool,
    end_sender: crossbeam_channel::Sender<()>,
) {
    thread::spawn(move || {
        EndChecker::new(
            requesters_wait_mutexes,
            requesters_pool,
            response_handlers_wait_mutexes,
            response_handlers_pool,
            paths_provider_wait_mutex,
            paths_provider_pool,
            end_sender,
        )
        .run();
    });
}

pub struct Discoverer {
    pub result_channel_receiver: ResultReceiver,
    pub end_channel_receiver: Receiver<()>,
}

impl Discoverer {
    fn new(
        result_channel_receiver: ResultReceiver,
        end_channel_receiver: Receiver<()>,
    ) -> Self {
        return Self {
            result_channel_receiver,
            end_channel_receiver,
        };
    }
}
