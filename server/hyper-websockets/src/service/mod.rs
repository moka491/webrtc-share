use hyper::{service::Service, Body, Request, Response};

use crate::error::WsResult;
use crate::handler::{HandlerMap, MessageHandlerFn};
use std::clone;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

mod request;

pub struct WebsocketConnectionService {
    handlers: Arc<HandlerMap>,
}

impl Service<Request<Body>> for WebsocketConnectionService {
    type Response = Response<Body>;
    type Error = crate::error::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<WsResult<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let resp = request::handle_request(req, self.handlers.clone());

        Box::pin(resp)
    }
}

pub struct WebsocketServiceBuilder {
    handlers: HandlerMap,
}

impl WebsocketServiceBuilder {
    fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn with_handler(mut self, event: &str, handler: MessageHandlerFn) -> Self {
        self.handlers.insert(event.to_string(), handler);
        self
    }

    pub fn build(self) -> WebsocketService {
        WebsocketService {
            handlers: Arc::new(self.handlers),
        }
    }
}

pub struct WebsocketService {
    handlers: Arc<HandlerMap>,
}

impl WebsocketService {
    pub fn builder() -> WebsocketServiceBuilder {
        WebsocketServiceBuilder::new()
    }
}

impl<T> Service<T> for WebsocketService {
    type Response = WebsocketConnectionService;
    type Error = crate::error::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<WsResult<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let handlers = self.handlers.clone();

        let fut = async move { Ok(WebsocketConnectionService { handlers }) };
        Box::pin(fut)
    }
}
