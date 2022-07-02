use futures::{sink::SinkExt, stream::StreamExt};
use hyper::service::Service;
use hyper::{Body, Request, Response};
use hyper_tungstenite::tungstenite::Message;
use hyper_tungstenite::HyperWebsocket;

use crate::error::Result;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

async fn handle_request(request: Request<Body>) -> Result<Response<Body>> {
    // Check if the request is a websocket upgrade request.
    if hyper_tungstenite::is_upgrade_request(&request) {
        let (response, websocket) = hyper_tungstenite::upgrade(request, None)?;

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = serve_websocket(websocket).await {
                eprintln!("Error in websocket connection: {}", e);
            }
        });

        // Return the response so the spawned future can continue.
        Ok(response)
    } else {
        // Handle regular HTTP requests here.
        Ok(Response::new(Body::from("Hello HTTP!")))
    }
}

/// Handle a websocket connection.
async fn serve_websocket(websocket: HyperWebsocket) -> Result<()> {
    let mut websocket = websocket.await?;

    while let Some(message) = websocket.next().await {
        match message? {
            Message::Text(msg) => {
                println!("Received text message: {}", msg);
                websocket
                    .send(Message::text("Thank you, come again."))
                    .await?;
            }
            _ => (),
        }
    }

    Ok(())
}

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
        let resp = handle_request(req);

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
