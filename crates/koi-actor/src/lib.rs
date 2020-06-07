//! Support for providing the actor-model paradigm for better managing
//! asynchronous notifications and events in a predictable manner.

use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

pub trait Actor {
    type InMessage: Send;

    fn poll_messages(&mut self, messages: &mut VecDeque<Self::InMessage>);
}

pub struct Stream<Message: Send + 'static> {
    pub channel: Sender<Message>,
}

pub fn spawn<A: Actor + Send + 'static>(mut actor: A) -> Stream<A::InMessage> {
    let (actor_tx, actor_rx) = channel();
    let mut message_queue = VecDeque::new();

    thread::spawn(move || loop {
        match push_pending_messages(&actor_rx, &mut message_queue) {
            Ok(_) => actor.poll_messages(&mut message_queue),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                eprintln!("Error: connection to channel has been disconnected.");
                break;
            }
        }
    });

    Stream { channel: actor_tx }
}

fn push_pending_messages<T>(rx: &Receiver<T>, queue: &mut VecDeque<T>) -> Result<(), TryRecvError> {
    if queue.is_empty() {
        match rx.recv() {
            Ok(m) => queue.push_back(m),
            Err(_) => return Err(TryRecvError::Disconnected)
        }
    }

    loop {
        match rx.try_recv() {
            Ok(m) => queue.push_back(m),
            Err(TryRecvError::Empty) => break Ok(()),
            Err(TryRecvError::Disconnected) => break Err(TryRecvError::Disconnected)
        }
    }
}
