mod dispatcher;
mod handler;

use self::dispatcher::{NotificationDispatcher, RequestDispatcher};
use crate::error::ProtocolError;
use crate::protocol::{Message, Notification, Request};
use crate::state::State;
use crate::Result;
use flume::Receiver;

pub struct Server<'a> {
    did_initialize: bool,
    receiver: Receiver<Message>,
    state: &'a mut State,
}

impl<'a> Server<'a> {
    pub fn new(receiver: Receiver<Message>, state: &'a mut State) -> Self {
        Self {
            did_initialize: false,
            receiver,
            state,
        }
    }

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

    fn handle_request(&mut self, request: Request) -> Result<()> {
        RequestDispatcher::new(request, self.state)
            .on::<lsp_types::request::Initialize>(handler::initialize)?
            .on::<lsp_types::request::Shutdown>(handler::shutdown)?
            .on::<lsp_types::request::Completion>(handler::completion)?
            .finish();

        Ok(())
    }

    fn handle_notification(
        &mut self,
        notification: Notification,
    ) -> Result<()> {
        NotificationDispatcher::new(notification, self.state)
            .on::<lsp_types::notification::Initialized>(handler::initialized)
            .on::<lsp_types::notification::DidOpenTextDocument>(
                handler::did_open_text_document,
            )
            .on::<lsp_types::notification::DidChangeTextDocument>(
                handler::did_change_text_document,
            )
            .finish();

        Ok(())
    }
}
