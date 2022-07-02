use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use hyper_websockets::service::WebsocketService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(WebsocketService {});
    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}
