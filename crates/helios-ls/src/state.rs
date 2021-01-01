#![allow(dead_code)]

use crate::protocol::Message;
use flume::Sender;
use helios_query::HeliosDatabase;
use std::default::Default;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Loading,
    Ready,
    Error,
}

impl Default for Status {
    fn default() -> Self {
        Self::Loading
    }
}

pub struct State {
    pub(crate) sender: Sender<Message>,
    pub(crate) db: HeliosDatabase,
    pub(crate) status: Status,
}

impl State {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
            db: HeliosDatabase::default(),
            status: Status::default(),
        }
    }

    pub fn snapshot(&self) -> StateSnapshot {
        use salsa::ParallelDatabase;
        StateSnapshot {
            db: self.db.snapshot(),
        }
    }

    pub fn send(&mut self, message: impl Into<Message>) {
        self.sender
            .send(message.into())
            .expect("Failed to send response")
    }
}

pub struct StateSnapshot {
    pub(crate) db: salsa::Snapshot<HeliosDatabase>,
}
