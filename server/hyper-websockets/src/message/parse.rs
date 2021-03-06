use crate::{
    error::WsResult,
    message::{Message, PartialMessage, RawMessage},
};
use serde::{Deserialize, Serialize};

pub fn partial_from_raw_message(raw_text_msg: &str) -> WsResult<PartialMessage> {
    let partial_msg: PartialMessage = serde_json::from_str(raw_text_msg)?;

    Ok(partial_msg)
}

pub fn message_from_partial<T>(partial_msg: PartialMessage) -> WsResult<Message<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let params: T = serde_json::from_value(partial_msg.params)?;

    Ok(Message {
        id: partial_msg.id,
        event: partial_msg.event,
        params,
    })
}

pub fn to_raw_message<T>(msg: Message<T>) -> WsResult<RawMessage>
where
    T: Serialize,
{
    let serialized_msg = serde_json::to_string(&msg)?;

    Ok(RawMessage::Text(serialized_msg))
}
