use crate::discoverer::communication::WaitMutex;
use std::thread::sleep;
use std::time::Duration;
use threadpool::ThreadPool;

pub struct EndChecker {
    requesters_wait_mutexes: Vec<WaitMutex>,
    requesters_pool: ThreadPool,
    response_handlers_wait_mutexes: Vec<WaitMutex>,
    response_handlers_pool: ThreadPool,
    paths_provider_wait_mutex: WaitMutex,
    paths_provider_pool: ThreadPool,
    end_sender: crossbeam_channel::Sender<()>,
    round_counter: u8,
}

impl EndChecker {
    pub fn new(
        requesters_wait_mutexes: Vec<WaitMutex>,
        requesters_pool: ThreadPool,
        response_handlers_wait_mutexes: Vec<WaitMutex>,
        response_handlers_pool: ThreadPool,
        paths_provider_wait_mutex: WaitMutex,
        paths_provider_pool: ThreadPool,
        end_sender: crossbeam_channel::Sender<()>,
    ) -> Self {
        return Self {
            requesters_wait_mutexes,
            requesters_pool,
            response_handlers_wait_mutexes,
            response_handlers_pool,
            paths_provider_wait_mutex,
            paths_provider_pool,
            end_sender,
            round_counter: 0,
        };
    }

    pub fn run(&mut self) {
        self.wait_for_end();
        self.send_end();
    }

    fn wait_for_end(&mut self) {
        loop {
            sleep(Duration::from_millis(20));

            if self.is_any_requester_active() {
                self.round_counter = 0;
                continue;
            }

            if self.is_any_response_handler_active() {
                self.round_counter = 0;
                continue;
            }

            if self.is_path_provider_active() {
                self.round_counter = 0;
                continue;
            }

            self.round_counter += 1;
            if self.round_counter == 10 {
                break;
            }
        }
    }

    fn is_any_requester_active(&self) -> bool {
        if self.requesters_pool.active_count() == 0 {
            return false;
        }

        for requester_wait_mutex in self.requesters_wait_mutexes.iter() {
            if let Ok(mut is_waiting) = requester_wait_mutex.try_lock() {
                *is_waiting = false;
                return true;
            }
        }
        return false;
    }

    fn is_any_response_handler_active(&self) -> bool {
        let active_response_handlers =
            self.response_handlers_pool.active_count();
        if active_response_handlers == 0 {
            return false;
        }

        for response_handler_wait_mutex in
            self.response_handlers_wait_mutexes.iter()
        {
            if let Ok(mut is_waiting) = response_handler_wait_mutex.try_lock() {
                *is_waiting = false;
                return true;
            }
        }
        return false;
    }

    fn is_path_provider_active(&self) -> bool {
        if self.paths_provider_pool.active_count() == 0 {
            return false;
        }

        if let Ok(_block) = self.paths_provider_wait_mutex.try_lock() {
            return true;
        }

        return false;
    }

    fn send_end(&self) {
        self.end_sender
            .send(())
            .expect("EndChecker: error sending end");
    }
}
