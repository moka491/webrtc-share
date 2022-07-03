use tokio::sync::mpsc::Sender;

use crate::message::{Message, PartialMessage, RawMessage};

use serde::{Deserialize, Serialize};

use crate::error::WsResult;
use crate::message::parse::{message_from_partial, to_raw_message};

pub struct Context {
    message: PartialMessage,
    socket_tx: Sender<RawMessage>,
}

impl Context {
    pub fn from(message: PartialMessage, socket_tx: Sender<RawMessage>) -> Self {
        Self { message, socket_tx }
    }

    pub fn parse_message<T>(&self) -> WsResult<Message<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        message_from_partial(self.message.clone())
    }

    pub async fn reply<T>(&mut self, event_name: &str, params: T) -> WsResult<()>
    where
        T: Serialize,
    {
        let new_msg = Message {
            id: self.message.id,
            event: event_name.to_string(),
            params,
        };

        let raw_msg = to_raw_message(new_msg)?;

        self.socket_tx.send(raw_msg);

        Ok(())
    }
}
