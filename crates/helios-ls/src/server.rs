mod handler;

pub use self::handler::Handler;
use crate::connection::Connection;
use crate::error::ProtocolError;
use crate::protocol::*;
use crate::Result;

pub struct Server<H: Handler> {
    did_initialize: bool,
    connection: Connection,
    handler: H,
}

impl<H: Handler> Server<H> {
    pub fn new(connection: Connection, handler: H) -> Self {
        Self {
            did_initialize: false,
            connection,
            handler,
        }
    }

    pub fn initialize(mut self) -> Result<Self> {
        match self.connection.receiver.recv()? {
            Message::Request(request) if request.is_initialize() => {
                let params = serde_json::from_value(request.params)?;
                let result = self.handler.initialize(params);
                self.send(Response::new_ok(request.id, result))?;
                self.did_initialize = true;
            }
            message => {
                let message = format!(
                    "expected initialize request, but found {:?}",
                    message
                );

                return Err(ProtocolError(message).into());
            }
        }

        Ok(Self {
            did_initialize: true,
            ..self
        })
    }

    pub fn run(self) -> Result<()> {
        while let Ok(message) = self.connection.receiver.recv() {
            if !self.did_initialize {
                log::warn!("Server is not initialized. Waiting for `initialize` message...");
                continue;
            }

            log::info!("~> {:?}", message);
        }

        Ok(())
    }

    fn send(&self, message: impl Into<Message>) -> Result<()> {
        self.connection.sender.send(message.into())?;
        Ok(())
    }
}
