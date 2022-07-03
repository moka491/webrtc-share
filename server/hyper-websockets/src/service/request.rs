use std::sync::Arc;

use futures::{sink::SinkExt, stream::StreamExt};
use hyper::{Body, Request, Response};
use hyper_tungstenite::{tungstenite::Message, HyperWebsocket};

use crate::{error::WsResult, handler::HandlerMap};

pub async fn handle_request(
    request: Request<Body>,
    handlers: Arc<HandlerMap>,
) -> WsResult<Response<Body>> {
    // Check if the request is a websocket upgrade request.
    if hyper_tungstenite::is_upgrade_request(&request) {
        let (response, websocket) = hyper_tungstenite::upgrade(request, None)?;

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = serve_websocket(websocket, handlers).await {
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
pub async fn serve_websocket(websocket: HyperWebsocket, handlers: Arc<HandlerMap>) -> WsResult<()> {
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
