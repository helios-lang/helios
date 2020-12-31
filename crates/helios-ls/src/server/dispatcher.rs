use crate::protocol::{Message, Notification, Request, RequestId, Response};
use flume::Sender;
use serde::{de::DeserializeOwned, Serialize};

pub struct RequestDispatcher {
    request: Option<Request>,
    sender: Sender<Message>,
}

impl RequestDispatcher {
    pub fn new(
        request: impl Into<Option<Request>>,
        sender: Sender<Message>,
    ) -> Self {
        Self {
            request: request.into(),
            sender,
        }
    }

    pub fn on<R>(&mut self, handler: fn(R::Params) -> R::Result) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + Send + 'static,
        R::Result: Serialize + std::fmt::Debug + 'static,
    {
        let (id, params) = match self.parse_request::<R>() {
            Some(it) => it,
            _ => return self,
        };

        let result = handler(params);
        let response = Response::new_ok(id, result);
        self.sender.send(response.into()).expect("Failed to send");

        self
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

pub struct NotificationDispatcher {
    notification: Option<Notification>,
}

impl NotificationDispatcher {
    pub fn new(notification: impl Into<Option<Notification>>) -> Self {
        Self {
            notification: notification.into(),
        }
    }

    pub fn on<N>(&mut self, handler: fn(N::Params)) -> &mut Self
    where
        N: lsp_types::notification::Notification + 'static,
        N::Params: DeserializeOwned + Send + 'static,
    {
        let params = match self.parse_notification::<N>() {
            Some(it) => it,
            _ => return self,
        };

        handler(params);

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
