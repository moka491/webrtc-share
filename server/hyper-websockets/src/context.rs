use crate::message::{Message, RawMessage};

use futures::SinkExt;
use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use serde::Serialize;

use crate::error::Result;

pub struct Context {
    pub message: Message,
    stream: WebSocketStream<Upgraded>,
}

impl Context {
    pub async fn reply<T>(&mut self, event_name: &str, params: T) -> Result<()>
    where
        T: Serialize,
    {
        let serialized_params: serde_json::Value = serde_json::to_value(params)?;

        let new_msg = Message {
            id: self.message.id,
            event: event_name.to_string(),
            params: serialized_params,
        };

        let serialized_msg = serde_json::to_string(&new_msg)?;

        self.stream.send(RawMessage::Text(serialized_msg)).await?;

        Ok(())
    }
}
