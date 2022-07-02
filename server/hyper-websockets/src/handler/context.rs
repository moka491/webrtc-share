use crate::message::{Message, PartialMessage, RawMessage};

use futures::SinkExt;
use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use serde::Serialize;

use crate::error::Result;
use crate::message::parse::to_raw_message;

pub struct Context {
    pub message: PartialMessage,
    stream: WebSocketStream<Upgraded>,
}

impl Context {
    pub async fn reply<T>(&mut self, event_name: &str, params: T) -> Result<()>
    where
        T: Serialize,
    {
        let new_msg = Message {
            id: self.message.id,
            event: event_name.to_string(),
            params,
        };

        let raw_msg = to_raw_message(new_msg)?;

        self.stream.send(raw_msg).await?;

        Ok(())
    }
}
