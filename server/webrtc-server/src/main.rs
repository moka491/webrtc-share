use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use hyper_websockets::error::WsResult;
use hyper_websockets::handler::context::Context;
use hyper_websockets::message::Message;
use hyper_websockets::service::WebsocketService;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Params {
    text: String,
}

fn hello_handler(ctx: Context) -> Pin<Box<dyn Future<Output = WsResult<()>>>> {
    Box::pin(async move {
        let msg: Message<Params> = ctx.parse_message()?;

        ctx.reply("world", ()).await;

        Ok(())
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let ws_service: WebsocketService = WebsocketService::builder()
        .with_handler("hello", Box::new(hello_handler))
        .build();

    let server = Server::bind(&addr).serve(ws_service);
    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}
