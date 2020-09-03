use crossbeam_channel::Receiver;
use std::sync::Arc;
use std::thread;
use threadpool::ThreadPool;

use crate::discoverer::actors::{
    EndChecker, Requester, ResponseHandler, UrlAggregator,
};
use crate::discoverer::communication::result_channel::{
    ResultChannel, ResultReceiver,
};
use crate::discoverer::communication::{
    new_wait_mutex, new_wait_mutex_vec, EndChannel, ResponseChannel,
    UrlChannel, UrlsChannel, WaitMutex,
};
use crate::discoverer::http::HttpOptions;
use crate::discoverer::scraper::{
    EmptyScraperProvider, ScraperProvider, UrlsScraperProvider,
};
use crate::discoverer::verificator;
use crate::discoverer::verificator::Verificator;

use reqwest::Url;

pub struct DiscovererBuilder {
    requesters_count: usize,
    response_handlers_count: usize,
    http_options: HttpOptions,
    response_verificator: Verificator,
    use_scraper: bool,
    base_urls: Vec<Url>,
    paths: Vec<String>,
}

impl DiscovererBuilder {
    pub fn new(base_urls: Vec<Url>, paths: Vec<String>) -> Self {
        return Self {
            requesters_count: 10,
            response_handlers_count: 10,
            http_options: HttpOptions::default(),
            response_verificator: verificator::create_default(),
            use_scraper: false,
            base_urls,
            paths,
        };
    }

    pub fn verificator(mut self, verificator: Verificator) -> Self {
        self.response_verificator = verificator;
        return self;
    }

    pub fn http_options(mut self, http_options: HttpOptions) -> Self {
        self.http_options = http_options;
        return self;
    }

    pub fn requesters_count(mut self, requesters_count: usize) -> Self {
        self.requesters_count = requesters_count;
        return self;
    }

    pub fn use_scraper(mut self, use_scraper: bool) -> Self {
        self.use_scraper = use_scraper;
        return self;
    }

    pub fn spawn(self) -> Discoverer {
        let response_verificator = Arc::new(self.response_verificator);
        let url_client = Arc::new(self.http_options.into());

        let requesters_pool = ThreadPool::with_name(
            "Requesters".to_string(),
            self.requesters_count,
        );

        let response_handlers_pool = ThreadPool::with_name(
            "Responsers".to_string(),
            self.response_handlers_count,
        );

        return DiscovererSpawner::new(
            response_verificator,
            url_client,
            requesters_pool,
            response_handlers_pool,
            self.use_scraper,
            self.base_urls,
            self.paths,
        )
        .spawn();
    }
}

struct DiscovererSpawner {
    response_verificator: Arc<Verificator>,
    url_client: Arc<reqwest::Client>,
    requesters_pool: ThreadPool,
    response_handlers_pool: ThreadPool,
    paths_provider_pool: ThreadPool,
    result_channel: ResultChannel,
    end_channel: EndChannel,
    response_channel: ResponseChannel,
    url_channel: UrlChannel,
    urls_channel: UrlsChannel,
    use_scraper: bool,
    base_urls: Vec<Url>,
    paths: Vec<String>,
}

impl DiscovererSpawner {
    fn new(
        response_verificator: Arc<Verificator>,
        url_client: Arc<reqwest::Client>,
        requesters_pool: ThreadPool,
        response_handlers_pool: ThreadPool,
        use_scraper: bool,
        base_urls: Vec<Url>,
        paths: Vec<String>,
    ) -> Self {
        let max_paths_count = requesters_pool.max_count() * 4;

        return Self {
            response_verificator,
            url_client,
            requesters_pool,
            response_handlers_pool,
            paths_provider_pool: ThreadPool::with_name(
                "Providers".to_string(),
                1,
            ),
            result_channel: ResultChannel::default(),
            end_channel: EndChannel::default(),
            response_channel: ResponseChannel::default(),
            url_channel: UrlChannel::with_capacity(max_paths_count),
            urls_channel: UrlsChannel::default(),
            use_scraper,
            base_urls,
            paths,
        };
    }

    fn spawn(mut self) -> Discoverer {
        let scraper = self.create_scraper();
        let response_handlers_wait_mutexes =
            new_wait_mutex_vec(self.response_handlers_pool.max_count());

        let requesters_wait_mutexes =
            new_wait_mutex_vec(self.requesters_pool.max_count());

        let paths_provider_wait_mutex = new_wait_mutex();

        self.spawn_response_handlers(&response_handlers_wait_mutexes, scraper);
        self.spawn_requesters(&requesters_wait_mutexes);
        self.spawn_url_aggregator(
            paths_provider_wait_mutex.clone(),
            self.base_urls.clone(),
            self.paths.clone(),
        );

        Self::spawn_end_checker(
            requesters_wait_mutexes,
            self.requesters_pool,
            response_handlers_wait_mutexes,
            self.response_handlers_pool,
            paths_provider_wait_mutex,
            self.paths_provider_pool,
            self.end_channel.sender.clone(),
        );

        return Discoverer::new(self.result_channel, self.end_channel);
    }

    fn create_scraper(&self) -> Box<dyn ScraperProvider> {
        match self.use_scraper {
            true => return Box::new(UrlsScraperProvider::new()),
            false => return Box::new(EmptyScraperProvider::new()),
        }
    }

    fn spawn_response_handlers(
        &mut self,
        wait_mutexes: &Vec<WaitMutex>,
        scraper: Box<dyn ScraperProvider>,
    ) {
        let scraper_arc = Arc::new(scraper);
        for (i, wait_mutex) in wait_mutexes.iter().enumerate() {
            self.spawn_response_handler(
                wait_mutex.clone(),
                i,
                scraper_arc.clone(),
            );
        }
    }

    fn spawn_response_handler(
        &mut self,
        wait_mutex: WaitMutex,
        id: usize,
        scraper: Arc<Box<dyn ScraperProvider>>,
    ) {
        let response_receiver = self.response_channel.receiver.clone();
        let result_sender = self.result_channel.sender.clone();
        let response_verificator = self.response_verificator.clone();

        self.response_handlers_pool.execute(move || {
            ResponseHandler::new(
                response_receiver,
                result_sender,
                response_verificator,
                scraper,
                wait_mutex,
                id,
            )
            .run();
        });
    }

    fn spawn_requesters(&mut self, wait_mutexes: &Vec<WaitMutex>) {
        for (i, wait_mutex) in wait_mutexes.iter().enumerate() {
            self.spawn_requester(wait_mutex.clone(), i);
        }
    }

    fn spawn_requester(&mut self, wait_mutex: WaitMutex, requester_id: usize) {
        let url_receiver = self.url_channel.receiver.clone();
        let client = self.url_client.clone();
        let response_sender = self.response_channel.sender.clone();

        self.requesters_pool.execute(move || {
            Requester::new(
                client,
                url_receiver,
                response_sender,
                wait_mutex,
                requester_id,
            )
            .run();
        });
    }

    fn spawn_url_aggregator(
        &self,
        wait_mutex: WaitMutex,
        base_urls: Vec<Url>,
        paths: Vec<String>,
    ) {
        let url_sender = self.url_channel.sender.clone();
        let scraper_receiver = self.urls_channel.receiver.clone();

        self.paths_provider_pool.execute(move || {
            UrlAggregator::new(url_sender, scraper_receiver, wait_mutex)
                .run(base_urls, paths);
        });
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
}

pub struct Discoverer {
    result_channel: ResultChannel,
    end_channel: EndChannel,
}

impl Discoverer {
    fn new(result_channel: ResultChannel, end_channel: EndChannel) -> Self {
        return Self {
            result_channel,
            end_channel,
        };
    }

    pub fn result_receiver(&self) -> &ResultReceiver {
        return &self.result_channel.receiver;
    }

    pub fn end_receiver(&self) -> &Receiver<()> {
        return &self.end_channel.receiver;
    }
}
