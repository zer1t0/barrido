use crossbeam_channel::{unbounded, Receiver, Sender};
use getset::Getters;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct EndChannel {
    receiver: Receiver<()>,
    sender: Sender<()>,
}

impl Default for EndChannel {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<()>();
        return Self { sender, receiver };
    }
}
