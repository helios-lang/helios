mod dispatcher;
mod handlers;

use self::dispatcher::{NotificationDispatcher, RequestDispatcher};
use crate::error::ProtocolError;
use crate::protocol::{Message, Notification, Request};
use crate::state::State;
use crate::Result;
use flume::Receiver;

/// The server side of the language server connection.
pub struct Server<'a> {
    did_initialize: bool,
    receiver: Receiver<Message>,
    state: &'a mut State,
}

impl<'a> Server<'a> {
    /// Constructs a new `Server` with the given receiver channel and state.
    pub fn new(receiver: Receiver<Message>, state: &'a mut State) -> Self {
        Self {
            did_initialize: false,
            receiver,
            state,
        }
    }

    /// Initializes a connection between the server and client, erroring if the
    /// server doesn't receive an `initialize` request from the client or it
    /// fails to send an `initialized` response.
    pub fn initialize(mut self) -> Result<Self> {
        match self.receiver.recv()? {
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

    /// Starts the main loop of the server.
    pub fn run(mut self) -> Result<()> {
        while let Ok(message) = self.receiver.recv() {
            if !self.did_initialize {
                log::warn!(
                    "Cannot process received message because the connection to \
                     the client has not been properly initialized. Waiting for \
                     the `initialize` message..."
                );
                continue;
            }

            match message {
                Message::Request(r) => self.handle_request(r)?,
                Message::Notification(n) if n.is_exit() => {
                    log::trace!("Exiting...");
                    break;
                }
                Message::Notification(n) => self.handle_notification(n)?,
                _ => log::info!("Unhandled message: {:?}", message),
            }
        }

        Ok(())
    }

    fn handle_request(&mut self, req: Request) -> Result<()> {
        use lsp_types::request::*;
        RequestDispatcher::new(req, self.state)
            .on::<Initialize>(handlers::initialize)?
            .on::<Shutdown>(handlers::shutdown)?
            .on::<Completion>(handlers::completion)?
            .on::<HoverRequest>(handlers::hover)?
            .finish();

        Ok(())
    }

    fn handle_notification(&mut self, not: Notification) -> Result<()> {
        use lsp_types::notification::*;
        NotificationDispatcher::new(not, self.state)
            .on::<Initialized>(handlers::initialized)
            .on::<DidOpenTextDocument>(handlers::did_open_text_document)
            .on::<DidChangeTextDocument>(handlers::did_change_text_document)
            .on::<DidSaveTextDocument>(handlers::did_save_text_document)
            .on::<DidChangeConfiguration>(handlers::did_change_configuration)
            .finish();

        Ok(())
    }
}
