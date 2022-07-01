use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_websocket::{handler::ConnHandler, message::Message};

async fn ws_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let conn = ConnHandler::builder()
        .add_handler("hello", ws_handler)
        .build();

    ws::start(conn, &req, stream)
}

fn ws_handler(msg: Message) {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(ws_route)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
