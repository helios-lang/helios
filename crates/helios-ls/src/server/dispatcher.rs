use crate::protocol::{Notification, Request, RequestId, Response};
use crate::state::{State, StateSnapshot};
use crate::Result;
use serde::{de::DeserializeOwned, Serialize};

pub struct RequestDispatcher<'a> {
    request: Option<Request>,
    state: &'a mut State,
}

impl<'a> RequestDispatcher<'a> {
    pub fn new(
        request: impl Into<Option<Request>>,
        state: &'a mut State,
    ) -> Self {
        Self {
            request: request.into(),
            state,
        }
    }

    pub fn on<R>(
        &mut self,
        handler: fn(StateSnapshot, R::Params) -> Result<R::Result>,
    ) -> Result<&mut Self>
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + Send + 'static,
        R::Result: Serialize + std::fmt::Debug + 'static,
    {
        let (id, params) = match self.parse_request::<R>() {
            Some(it) => it,
            _ => return Ok(self),
        };

        // TODO: Move computation to separate thread and allow for error
        // recovery (and possibly sending an error response back to the client).
        let snapshot = self.state.snapshot();
        let result = handler(snapshot, params)?;
        let response = Response::new_ok(id, result);
        self.state.send(response);

        Ok(self)
    }

    pub fn finish(&mut self) {
        if let Some(request) = self.request.take() {
            log::warn!("Unhandled request: {:?}", request);
        }
    }

    fn parse_request<R>(&mut self) -> Option<(RequestId, R::Params)>
    where
        R: lsp_types::request::Request,
    {
        let request = match &self.request {
            Some(request) if request.method == R::METHOD => {
                self.request.take().unwrap()
            }
            _ => return None,
        };

        let params = serde_json::from_value(request.params).unwrap();
        Some((request.id, params))
    }
}

pub struct NotificationDispatcher<'a> {
    notification: Option<Notification>,
    state: &'a mut State,
}

impl<'a> NotificationDispatcher<'a> {
    pub fn new(
        notification: impl Into<Option<Notification>>,
        state: &'a mut State,
    ) -> Self {
        Self {
            notification: notification.into(),
            state,
        }
    }

    pub fn on<N>(&mut self, handler: fn(&mut State, N::Params)) -> &mut Self
    where
        N: lsp_types::notification::Notification + 'static,
        N::Params: DeserializeOwned + Send + 'static,
    {
        let params = match self.parse_notification::<N>() {
            Some(it) => it,
            _ => return self,
        };

        handler(self.state, params);

        self
    }

    pub fn finish(&mut self) {
        if let Some(notification) = self.notification.take() {
            log::warn!("Unhandled notification: {:?}", notification);
        }
    }

    fn parse_notification<N>(&mut self) -> Option<N::Params>
    where
        N: lsp_types::notification::Notification,
    {
        let notification = match &self.notification {
            Some(notification) if notification.method == N::METHOD => {
                self.notification.take().unwrap()
            }
            _ => return None,
        };

        let params = serde_json::from_value(notification.params).unwrap();
        Some(params)
    }
}
