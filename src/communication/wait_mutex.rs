use std::sync::Arc;
use std::sync::Mutex;

pub type WaitMutex = Arc<Mutex<bool>>;

pub fn new_wait_mutex() -> WaitMutex {
    return Arc::new(Mutex::new(false));
}

pub fn new_wait_mutex_vec(count: usize) -> Vec<WaitMutex> {
    let mut wait_mutexes = Vec::with_capacity(count);
    for _ in 0..count {
        wait_mutexes.push(self::new_wait_mutex());
    }
    return wait_mutexes;
}
