use crossbeam_channel::{bounded, unbounded};
pub use crossbeam_channel::{Receiver, Sender};
use getset::Getters;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
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
