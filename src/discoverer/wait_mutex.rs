use std::sync::Arc;
use std::sync::Mutex;

pub type WaitMutex = Arc<Mutex<bool>>;

pub fn new() -> WaitMutex {
    return Arc::new(Mutex::new(false));
}

pub fn new_vec(count: usize) -> Vec<WaitMutex> {
    let mut wait_mutexes = Vec::with_capacity(count);
    for _ in 0..count {
        wait_mutexes.push(self::new());
    }
    return wait_mutexes;
}