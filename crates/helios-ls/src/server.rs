mod dispatcher;
mod handler;

use crate::error::ProtocolError;
use crate::protocol::{Message, Request};
use crate::Result;
use crate::{connection::Connection, protocol::Notification};
use dispatcher::{NotificationDispatcher, RequestDispatcher};

pub struct Server {
    did_initialize: bool,
    connection: Connection,
}

impl Server {
    pub fn new(connection: Connection) -> Self {
        Self {
            did_initialize: false,
            connection,
        }
    }

    pub fn initialize(mut self) -> Result<Self> {
        match self.connection.receiver.recv()? {
            Message::Request(request) if request.is_initialize() => {
                self.handle_request(request)?;
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

            match message {
                Message::Request(request) => self.handle_request(request)?,
                Message::Notification(notification) => {
                    self.handle_notification(notification)?
                }
                _ => log::info!("Unhandled message: {:?}", message),
            }
        }

        Ok(())
    }

    fn handle_request(&self, request: Request) -> Result<()> {
        RequestDispatcher::new(request, self.connection.sender.clone())
            .on::<lsp_types::request::Initialize>(handler::initialize)
            .on::<lsp_types::request::Shutdown>(handler::shutdown)
            .finish();

        Ok(())
    }

    fn handle_notification(&self, notification: Notification) -> Result<()> {
        NotificationDispatcher::new(notification)
            .on::<lsp_types::notification::Initialized>(handler::initialized)
            .finish();

        Ok(())
    }
}
