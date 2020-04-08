use crossbeam_channel::{bounded, unbounded};
pub use crossbeam_channel::{Receiver, Sender};
use getset::Getters;

#[derive(Getters)]
#[getset(get = "pub with_prefix")]
pub struct Channel<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<T>();
        return Self { sender, receiver };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = bounded::<T>(capacity);
        return Self { sender, receiver };
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        return Self::new();
    }
}
