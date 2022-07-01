use std::collections::HashMap;

use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

use crate::{message::Message, message_handler::MessageHandlerFn};

pub struct ConnHandlerBuilder {
    handlers: HashMap<String, MessageHandlerFn>,
}

impl ConnHandlerBuilder {
    fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(self, msg_name: &str, handler_fn: MessageHandlerFn) -> Self {
        self
    }

    pub fn build(self) -> ConnHandler {
        ConnHandler {
            handlers: self.handlers,
        }
    }
}

pub struct ConnHandler {
    pub handlers: HashMap<String, MessageHandlerFn>,
}

impl ConnHandler {
    pub fn builder() -> ConnHandlerBuilder {
        ConnHandlerBuilder::new()
    }
}

impl Actor for ConnHandler {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ConnHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            serde_json::from_str(&text)
                .map_err(|err| err)
                .ok()
                .and_then(|msg: Message| Some((self.handlers.get(&msg.msg_type)?, msg)))
                .and_then(|(handler, msg)| {
                    handler(msg);
                    Some(())
                });
        }
    }
}
