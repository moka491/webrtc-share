use futures::{sink::SinkExt, stream::StreamExt};
use hyper::service::Service;
use hyper::{Body, Request, Response};
use hyper_tungstenite::tungstenite::Message;
use hyper_tungstenite::HyperWebsocket;

use crate::error::Result;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

mod request;

pub struct WsConnService {}

impl Service<Request<Body>> for WsConnService {
    type Response = Response<Body>;
    type Error = crate::error::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let resp = request::handle_request(req);

        Box::pin(resp)
    }
}

pub struct WebsocketService {}

impl<T> Service<T> for WebsocketService {
    type Response = WsConnService;
    type Error = crate::error::Error;
    type Future =
        Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let fut = async move { Ok(WsConnService {}) };
        Box::pin(fut)
    }
}
