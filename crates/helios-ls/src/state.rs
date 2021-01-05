use crate::protocol::Message;
use flume::Sender;
use helios_query::HeliosDatabase;
use std::default::Default;

/// The current status of the language server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Loading,
    #[allow(unused)]
    Ready,
    #[allow(unused)]
    Error,
}

impl Default for Status {
    fn default() -> Self {
        Self::Loading
    }
}

/// The shared state of the server side of the language server.
pub struct State {
    /// The sender channel.
    pub(crate) sender: Sender<Message>,
    /// The current status.
    #[allow(unused)]
    pub(crate) status: Status,
    /// The `salsa` database for computing and caching queries.
    pub(crate) db: HeliosDatabase,
}

impl State {
    /// Constructs a new `State` with the given sender channel.
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            sender,
            db: HeliosDatabase::default(),
            status: Status::default(),
        }
    }

    /// Returns a snapshot of the database for multithreaded operations.
    pub fn snapshot(&self) -> StateSnapshot {
        use salsa::ParallelDatabase;
        StateSnapshot {
            db: self.db.snapshot(),
        }
    }

    /// Sends a message to the client.
    pub fn send(&mut self, message: impl Into<Message>) {
        self.sender
            .send(message.into())
            .expect("Failed to send response")
    }
}

/// A snapshot of the `salsa` database.
pub struct StateSnapshot {
    #[allow(unused)]
    db: salsa::Snapshot<HeliosDatabase>,
}
